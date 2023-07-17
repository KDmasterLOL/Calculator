export function input_process() {
	let inputs = document.querySelectorAll("input");
	for (let input of inputs) {
		input.addEventListener("input", () => {
			input.style.width = `${input.value.length + 1}ch`;
		})
	}

	let evaluate = document.querySelector("#play_audio");
	let scrimer = document.querySelector("#scrimer")
	scrimer.classList.toggle("hidden");
	evaluate.addEventListener("click", () => {
		let au = new Audio("https://assets.mixkit.co/active_storage/sfx/309/309-preview.mp3");
		scrimer.classList.toggle("hidden");
		au.play();
	})
}
