# orderly-dashboard-FE
## Install deps
```shell
npm install
```
## Prerequisites
set env variables
```
echo 'PORT=3000\nANALYZER_SERVER_ADDR=http://localhost:8089\n' > .env
```
## Start
MacOS or Linux
```shell
DEBUG=myapp:* npm start
```
Window
```shell
set DEBUG=myapp:* & npm start
```
Visit on explorer  
http://localhost:3000/example.html
http://localhost:3000/echart-example.html
## Chart getting started
* [Create a Chart](https://www.chartjs.org/docs/latest/getting-started/#create-a-chart)
## Dependencies
### Front END Framework
[Express](https://www.expressjs.com.cn/starter/generator.html)
### Chart plugin
[chart.js](https://www.chartjs.org/)
[echarts](https://echarts.apache.org/examples/zh/index.html)
### Boostrap5
[Bootstrap](https://getbootstrap.com/docs/5.3/layout/containers/)