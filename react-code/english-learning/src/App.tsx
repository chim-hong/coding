import React, { useState } from 'react'
import { Input, Button, Space } from 'antd'
import { GrammarlyEditorPlugin, GrammarlyButton } from '@grammarly/editor-sdk-react'
import { Configuration, OpenAIApi } from "openai";
const configuration = new Configuration({
  organization: "org-gy6XG3og47WgbJYWsKHegCpW",
  apiKey: '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',
});
const openai = new OpenAIApi(configuration);

function GrammarlyEditor() {
  const [answers, setAnswers] = useState<{ type: 'prompt' | 'answer', message: string | any }[]>([])
  const [input, setInput] = useState<string>('')


  async function fetchAnswer() {
    setAnswers(answers => {
      return [
        ...answers,
        { type: 'prompt', message: input }
      ]
    })
    const response = await openai.createChatCompletion({
      model: "gpt-3.5-turbo",
      messages: [
        { "role": "system", "content": "You are a helpful assistant that English teacher, and your name is GPT-123. " },
        { "role": "system", "content": "When you introduce yourself, you should tell me your mission." },
        { "role": "system", "content": "if there is a mistake, you need to tell me the mistake first when I sended message to you. " },
        { "role": "system", "content": "If the question I asked was a 'Chinese English', you needs tell me 'It's Chinese English/Chinlish' and correct it." },
        { role: 'user', content: input }],
      temperature: 0,
      max_tokens: 70,
    });
    if (response) {
      setAnswers(answers => {
        return [
          ...answers,
          { type: 'answer', message: response.data.choices[0].message!.content }
        ]
      })
      setInput('')
    }
  }

  return (
    <>
      <div style={{ width: 700, height: 500, overflow: 'auto',border:'1px solid #999',padding:20 }} >
        {
          answers.map((item) => (<div
            style={{ padding: '20px 0' }}>
            {item.type === 'answer'
              ?
              <div>{`Mr.Teacher: ${item.message}`}</div>
              :
              <div style={{ textAlign: 'end' }}>{`${item.message}: you`}</div>
            }
          </div>))
        }
      </div>
      <Space>
        <GrammarlyEditorPlugin
          clientId="client_1wDpXD29PZoRwWowZu2MKs"
          config={{
            activation: 'immediate'
          }}
          onKeyDownCapture={(e) => {
            if (e.code === 'Enter') {
              fetchAnswer();
            }
          }}
        >
          <Input
            value={input}
            onChange={(e) => {
              setInput(e.target.value)
            }}
          />
        </GrammarlyEditorPlugin>
        <Button
          onClick={fetchAnswer}
        >Submit
        </Button>
      </Space>
    </>
  )
}
export default GrammarlyEditor;
