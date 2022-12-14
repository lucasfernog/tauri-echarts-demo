<script>
import { onMounted, ref, watch } from 'vue';
import { mountChart, updateChart } from './composable/chart.js';

export default {
  name: 'Chart Demo',
  setup() {
    const refreshSpeed = ref(120);
    const numberOfPoints = ref(2_000);
    const running = ref(true);

    function start() {
      running.value = true;
    }

    function stop() {
      running.value = false;
    }

    onMounted(() => {
      const myChart = mountChart(document.getElementById('main'), numberOfPoints.value);

      window.onresize = () => {
        myChart.resize();
      };

      let intervalId;

      watch([running, refreshSpeed, numberOfPoints], ([running, speed, points]) => {
        if (intervalId) {
          window.clearInterval(intervalId);
        }
        if (running) {
          intervalId = window.setInterval(() => {
            updateChart(myChart, points);
          }, 60 / speed * 1000);
        }
      }, { immediate: true });
    });

    return {
      refreshSpeed,
      numberOfPoints,
      running,

      start,
      stop,
    }
  },
};
</script>

<template>
  <div class="row items-center">
    <div class="column">
      <div class="row justify-between items-center gap-20">
        <label>Refresh Speed (Hz)</label>
        <input v-model="refreshSpeed">
      </div>
      <div class="row justify-between items-center gap-20">
        <label>Number of points</label>
        <input v-model="numberOfPoints">
      </div>
      <div class="row">
        <button class="chartButton" :disabled="running" @click="start">Start</button>
        <button class="chartButton" :disabled="!running" @click="stop">Stop</button>
      </div>
    </div>
    <div class="chart-container">
      <div id="main" class="chart"></div>
    </div>
  </div>
</template>

<style>
.row {
  display: flex;
  flex-direction: row;
}

.justify-between {
  justify-content: space-between;
}

.items-center {
  align-items: center;
}

.gap-20 {
  gap: 20px;
}

.chartButton {
  width: 160px;
  margin-left: 10px;
  margin-right: 10px;
}

body {
  height: 100%;
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
}
#app {
  height: 100%;
  width: 100%;
  padding: 20px;
}
.chart-container {
  height: 600px;
  width: 600px;
  border-radius: 5px;
}
.chart {
  width: 100%;
  height: 100%;
}
</style>
