export const secondify = (timestamp: string): number =>
  new Date("1970-01-01T00:" + timestamp + "Z").getTime() / 1000
