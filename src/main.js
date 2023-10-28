const { invoke } = window.__TAURI__.tauri;

let mathInputEl;
let mathCorrectEl;
let mathQuestionEl;
let rangeEl;
let positiveEl;

async function submitAnswer() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let answers = await invoke("get_answer", {
    answer: mathInputEl.value,
    question: mathQuestionEl.textContent,
  });
  mathCorrectEl.textContent = answers[0];
  if (answers[1]) {
    newQuestion();
  }
  mathInputEl.focus();
  mathInputEl.select();
}

async function newQuestion() {
  mathQuestionEl.textContent = await invoke("get_new_question", {
    range: rangeEl.value,
  });
}

async function skipQuestion() {
  let answers = await invoke("skip_question", {
    question: mathQuestionEl.textContent,
    range: rangeEl.value,
  });
  mathCorrectEl.textContent = "The correct answer was: " + answers[0];
  mathQuestionEl.textContent = answers[1];
}

window.addEventListener("DOMContentLoaded", () => {
  mathInputEl = document.querySelector("#math-input");
  mathCorrectEl = document.querySelector("#math-correct");
  mathQuestionEl = document.querySelector("#math-question");
  rangeEl = document.querySelector("#range");
  positiveEl = document.querySelector("#positive");
  newQuestion();
  document.querySelector("#math-form").addEventListener("submit", (e) => {
    e.preventDefault();
    submitAnswer();
  });
  document.getElementById("range").addEventListener("change", function () {
    let v = parseInt(this.value);
    if (v > 2 && v < 1000000000) return
    else if (v > 1000000000) this.value = 1000000000;
    else this.value = 2;
  });
  document.querySelector("#range").addEventListener("change", (e) => {
    newQuestion();
  });
  document.querySelector("#skip").addEventListener("click", (e) => {
    skipQuestion();
  })
});