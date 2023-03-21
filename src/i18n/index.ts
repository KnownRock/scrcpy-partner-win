// ./src/Shared/lang/index.js
import { init, register, getLocaleFromNavigator } from 'svelte-i18n'

register('en', async () => await import('./en.json'))
register('zh-CN', async () => await import('./zh.json'))
// I have other locale files registered as well, but I've commented
// them out temporarily for the sake of trying to get this working

const loc = getLocaleFromNavigator()

console.log(loc)

void init({
  fallbackLocale: 'en',
  initialLocale: loc // 'en'
})
