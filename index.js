export function input_process() {
	let inputs = document.querySelectorAll("input");
	for (let input of inputs) {
		input.addEventListener("input", () => {
			input.style.width = `${input.value.length + 1}ch`;
		})
	}
}
