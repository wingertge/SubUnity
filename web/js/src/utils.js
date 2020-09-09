export const secondify = timestamp =>
  new Date("1970-01-01T00:" + timestamp + "Z").getTime() / 1000
