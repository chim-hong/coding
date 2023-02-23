const baiduData = await fetch('https://baidu.com/');

const data = baiduData.clone().json();

console.log('baiduData',data)
