import * as wasm from "phywasm"

const num = 8000
const engine = wasm.Engine.new(num)

function update_engine(time) {
  try {
    engine.update(time)
  } catch {}
  window.requestAnimationFrame(update_engine)
}

for (let i = 0; i < num; i++) engine.add_body(wasm.Vector.new(i, 32.0))
update_engine(performance.now())

let last = performance.now()
window.addEventListener("message", (msg) => {
  const data = JSON.parse(msg.data)
  const dt = performance.now() - last
  last = performance.now()

  console.clear()
  console.log(1000 / dt)
})
