import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type Counter = { 'topic' : string } |
  { 'value' : bigint };
export type Subscriber = { 'topic' : string };
export interface _SERVICE {
  'get_count' : ActorMethod<[], bigint>,
  'setup_subscribe' : ActorMethod<[Principal, string], undefined>,
  'update_count' : ActorMethod<[Counter], undefined>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
