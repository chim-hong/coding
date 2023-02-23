<script setup lang="ts">
import { reactive } from 'vue';

const isButtonDisabled = true;
const button = "button123"
const attribute = {
  disabled:true
}
const bar = 'onclick'
const value = () => {
  console.log('1234',1234)
}
let params = reactive({num:1,age:18})

const handleChangeParamsValue = () => {
  console.log('1',1)
  params={...params,num:22}
}

</script>

<template>
  <main>
    <button v-bind="attribute">{{button}}</button>
    <!-- 这会触发一个编译器警告 -->
    <a @click.prevent="value"> a标签 </a>
   <button @click="handleChangeParamsValue">change params value</button>
   {{params.num}}
  </main>
</template>
