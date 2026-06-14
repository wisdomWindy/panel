# 澄清记录

## question

页面“样式没生效”问题的直接修复范围是什么？

## answer

需要同时处理两部分：一是修复 Tailwind 在渲染端的接入方式，二是把 `src/renderer/**/*.vue` 中的页面样式约束统一为 Tailwind class，不允许 `<style>`、`style`、`:style`、`v-bind:style`。

## final decision

本次按渲染端全部 Vue 组件范围执行约束检查与治理；若代码本身已无残留样式标签或属性，则保持模板结构不做无意义改写，只修复供应链与记录约束。

## affected spec area

- in scope
- page and module design
- acceptance criteria
