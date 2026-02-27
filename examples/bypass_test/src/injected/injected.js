console.log("Injected JS executed!")
// document.body.style.backgroundColor = "#ff0000"
// document.body.innerHTML += "<h1 style='color:white; z-index:9999; position:fixed; top:0; left:0'>Injected JS executed!</h1>"
try {
  setInterval(() => {
    window.__TAURI__.core.invoke("plugin:restricted-plugin|log_from_injected", { msg: Date.now().toString() })
      .then(() => console.log("Command invoked successfully"))
      .catch(e => console.error("Command failed", e))
  }, 3000)
} catch (e) {
  console.error("Invoke not available", e)
}
