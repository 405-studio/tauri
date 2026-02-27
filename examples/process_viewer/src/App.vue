<script setup>
import { ref, onMounted } from 'vue'

const processes = ref([])
const loading = ref(false)
const error = ref(null)

// Tauri invoke function placeholder.
// In a real Tauri app, this is provided by window.__TAURI__.core.invoke
// We need to ensure we can access it.
const invoke = window.__TAURI__ ? window.__TAURI__.core.invoke : null

const fetchProcesses = async () => {
  if (!invoke) {
    error.value = "Tauri API not found. Are you running in Tauri?"
    return
  }
  
  loading.value = true
  error.value = null
  try {
    processes.value = await invoke('get_processes')
  } catch (e) {
    error.value = e.toString()
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchProcesses()
})
</script>

<template>
  <div class="container">
    <h1>Process Viewer</h1>
    
    <div class="controls">
      <button @click="fetchProcesses" :disabled="loading">
        {{ loading ? 'Refreshing...' : 'Refresh Processes' }}
      </button>
      <span v-if="processes.length > 0" class="count">
        Total: {{ processes.length }}
      </span>
    </div>

    <div v-if="error" class="error">
      {{ error }}
    </div>

    <div class="table-container">
      <table>
        <thead>
          <tr>
            <th>PID</th>
            <th>Name</th>
            <th>Memory (KB)</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="proc in processes" :key="proc.pid">
            <td>{{ proc.pid }}</td>
            <td>{{ proc.name }}</td>
            <td>{{ (proc.memory / 1024).toFixed(2) }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<style scoped>
.container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
  font-family: Arial, sans-serif;
}

.controls {
  margin-bottom: 1rem;
  display: flex;
  gap: 1rem;
  align-items: center;
}

.count {
  font-weight: bold;
}

.error {
  color: red;
  margin-bottom: 1rem;
  padding: 1rem;
  border: 1px solid red;
  background-color: #ffeeee;
}

.table-container {
  overflow-x: auto;
  border: 1px solid #ddd;
  border-radius: 4px;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th, td {
  padding: 12px;
  text-align: left;
  border-bottom: 1px solid #ddd;
}

th {
  background-color: #f5f5f5;
  font-weight: bold;
}

tr:hover {
  background-color: #f9f9f9;
}

button {
  padding: 8px 16px;
  background-color: #4CAF50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

button:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
}

button:hover:not(:disabled) {
  background-color: #45a049;
}
</style>
