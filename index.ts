export * from './rust-module.js'

export enum Button {
  Left,
  Middle,
  Right,
  ScrollUp,
  ScrollDown,
  ScrollLeft,
  ScrollRight,
}

export enum Direction {
  Press,
  Release,
  Click,
}

export enum Coordinate {
  Abs,
  Rel,
}

export enum Axis {
  Horizontal,
  Vertical,
}

export enum Key {
  Alt,
  Backspace,
  CapsLock,
  Control,
  Delete,
  DownArrow,
  End,
  Escape,
  F1,
  F10,
  F11,
  F12,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  Home,
  LeftArrow,
  Meta,
  Option,
  PageDown,
  PageUp,
  Return,
  RightArrow,
  Shift,
  Space,
  Tab,
  UpArrow,
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
}

export function sleep(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms))
}
