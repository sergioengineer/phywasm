import * as wasm from "phywasm"

const engine = wasm.Engine.new(32)

function update_engine(time) {
  try {
    engine.update(time)
  } catch {}
  window.requestAnimationFrame(update_engine)
}

engine.add_body(wasm.Vector.new(32.0, 32.0))
engine.add_body(wasm.Vector.new(32.0, 32.0))
update_engine(performance.now())

window.addEventListener("message", (msg, evt) => {
  const data = JSON.parse(msg.data)
  console.log(data.id, data.position.x, data.position.y)
})
