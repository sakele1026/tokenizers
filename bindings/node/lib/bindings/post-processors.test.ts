/* eslint-disable @typescript-eslint/no-explicit-any */

import { bertProcessing, byteLevelProcessing, robertaProcessing, sequenceProcessing, templateProcessing } from '../../'

describe('bertProcessing', () => {
  it('instantiates correctly with only two parameters', () => {
    const processor = bertProcessing(['sep', 1], ['cls', 2])
    expect(processor.constructor.name).toEqual('Processor')
  })

  it('throws if only one argument is provided', () => {
    expect(() => (bertProcessing as any)(['sep', 1])).toThrow('Given napi value is not an array')
  })

  it('throws if arguments are malformed', () => {
    expect(() => (bertProcessing as any)(['sep', '1'], ['cls', '2'])).toThrow(
      'Failed to convert napi value String into rust type `u32`',
    )
    expect(() => (bertProcessing as any)(['sep'], ['cls'])).toThrow('Array length < 2')
  })
})

describe('byteLevelProcessing', () => {
  it('instantiates correctly without any parameter', () => {
    const processor = byteLevelProcessing()
    expect(processor.constructor.name).toEqual('Processor')
  })

  it('accepts `undefined` as first parameter', () => {
    expect(byteLevelProcessing(undefined)).toBeDefined()
  })

  it('accepts `boolean` as first parameter', () => {
    expect(byteLevelProcessing(true)).toBeDefined()
  })
})

describe('robertaProcessing', () => {
  it('instantiates correctly with only two parameters', () => {
    const processor = robertaProcessing(['sep', 1], ['cls', 2])
    expect(processor.constructor.name).toEqual('Processor')
  })

  it('accepts `undefined` as third and fourth parameters', () => {
    expect(robertaProcessing(['sep', 1], ['cls', 2], undefined, undefined)).toBeDefined()
  })

  it('accepts `boolean` as third and fourth parameter', () => {
    expect(robertaProcessing(['sep', 1], ['cls', 2], true, true)).toBeDefined()
  })
})

describe('templateProcessing', () => {
  it('instantiates correctly with only a single template', () => {
    const processor = templateProcessing('$A $A')
    expect(processor.constructor.name).toEqual('Processor')
  })

  it('throws if special tokens are missing', () => {
    expect(() => templateProcessing('[CLS] $A [SEP]')).toThrow('Missing SpecialToken(s) with id(s)')
  })

  it('instantiates correctly with both templates', () => {
    const processor = templateProcessing('[CLS] $A [SEP]', '[CLS] $A [SEP] $B:1 [SEP]:1', [
      ['[CLS]', 1],
      ['[SEP]', 2],
    ])
    expect(processor.constructor.name).toEqual('Processor')
  })
})

describe('sequenceProcessing', () => {
  it('accepts `PostProcessor[]` as first parameter', () => {
    const template = templateProcessing('[CLS] $A [SEP]', '[CLS] $A [SEP] $B:1 [SEP]:1', [
      ['[CLS]', 1],
      ['[SEP]', 2],
    ])
    const bytelevel = byteLevelProcessing(true)
    expect(sequenceProcessing([bytelevel, template])).toBeDefined()
  })
})
