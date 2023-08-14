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
