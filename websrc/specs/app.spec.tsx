import { render } from '@testing-library/react'
import { describe, expect, test } from 'vitest'

import App from '../app'

describe('App test', () => {
  test('should render successfully', () => {
    const { baseElement } = render(<App />)
    expect(baseElement).toBeTruthy()
  })
})
