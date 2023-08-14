import { useState } from 'react'
import { Input } from 'antd'

import { createLanguageModel, createJsonTranslator } from './typechat/'
const model = createLanguageModel(
  {
    OPENAI_API_KEY: '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',
    OPENAI_MODEL: 'gpt-3.5-turbo'
  });

const schema =
  `
  export type Search = LineItem | UnknownText;

  // Use this type for order items that match nothing else
  export interface UnknownText {
    type: 'unknown',
    text: string; // The text that wasn't understood
  }

  export interface LineItem {
    params: Product;
  }

  export type Product = Decision | Channel;

  export interface Decision {
    Decision?: DecisionOptions[];
  }

  export interface Channel {
    Channel?: ChannelOptions[];
  }

  export type DecisionOptions = 'Accept' | 'Review' | 'Reject'

  export type ChannelOptions = 'ios' | 'android' | 'web'
`;


const translator = createJsonTranslator(model, schema, 'Search22');

function GrammarlyEditor() {
  const [answers, setAnswers] = useState<any>()
  const [input, setInput] = useState<string>('')

  async function fetchAnswer() {
    setAnswers("正在查找数据……");
    const response = await translator.translate(input) as any;
    console.log('response', response)
    setAnswers(response?.data?.params)
  }

  return (
    <>
      <div style={{ height: 200, overflow: 'auto', border: '1px solid #999', padding: 20 }} >
        {JSON.stringify(answers)}
      </div>
      <Input
        value={input}
        onChange={(e) => {
          setInput(e.target.value)
        }}
        onKeyDownCapture={(e) => {
          if (e.key === 'Enter') {
            fetchAnswer();
          }
        }}
      />
    </>
  )
}
export default GrammarlyEditor;
