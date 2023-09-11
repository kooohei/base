<script setup>
import {appWindow} from '@tauri-apps/api/window'

const minimize = () => appWindow.minimize()
const maximize = () => appWindow.maximze()
const close = () => appWindow.close()
</script>

<template lang="pug">
div.titlebar(data-tauri-drag-region)
  div.titlebar-button(@click="minimize")
    i.tsr.ts-chrome-minimize
  div.titlebar-button(@click="maximize")
    i.tsr.ts-chrome-maximize
  div.titlebar-button.close(@click="close")
    i.tsr.ts-chrome-close
</template>

<style lang="stylus" scoped>
div.titlebar
  background rgba(255,255,255,.7)
  display flex
  justify-content flex-end
  position fixed
  top 0
  left 0
  right 0

.titlebar-button
  display inline-flex
  justify-content center
  align-items center
  width 1.8rem
  height 1.8rem
  
  i
    font-size .5rem
    

.titlebar-button:hover
  background-color rgba(100,100,100,.4)
  &.close
    background-color rgba(255, 80, 80, 1)


</style>
