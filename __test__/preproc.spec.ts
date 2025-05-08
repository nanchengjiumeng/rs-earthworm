import { test, expect } from 'vitest'
import path from 'path'
import { fileURLToPath } from 'url'

import { filterBinaryzation } from '../index'

const __dirname = path.dirname(fileURLToPath(import.meta.url))

const horsesJpg = path.join(__dirname, '../test/horses.jpg')
const horsesFilterPng = path.join(__dirname, '../target/image/horses_filter.png')

test('filter binaryzation', () => {
  filterBinaryzation(horsesJpg, '0.5')
})
