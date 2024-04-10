import init, { run } from "./pkg/smcu_web.js"

let input = document.getElementById("asm")
let result = document.getElementById("res")

init().then(() => {
	console.log("Wasm initialized")
	input.onchange = () => {
		console.log("Changed")
		result.value = run(input.value)
	}
})
