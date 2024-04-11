import init, { run, Result, ResultType } from "./pkg/smcu_web.js"

let input = document.getElementById("asm")
let result = document.getElementById("res")

init().then(() => {
	console.log("Wasm initialized")
	input.onchange = () => {
		console.log("Changed")
		console.debug(input.value)
		let assembly = run(input.value)
		console.debug(assembly)
		
		if (assembly.res == ResultType.Ok) {
			console.debug("Assembled")
			result.value = assembly.data[0]
		} else {
			console.debug("Failed")

			result.value = ""
			for (let error in assembly.data) {
				console.log(assembly.data[error])
				result.value += assembly.data[error] + "\n"
			}
		}
	}
})
