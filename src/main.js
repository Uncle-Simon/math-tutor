const { invoke } = window.__TAURI__.tauri;

let mathInputEl;
let mathCorrectEl;
let mathQuestionEl;
let rangeEl;

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
  mathQuestionEl.textContent = await invoke("get_new_question", { range: rangeEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  mathInputEl = document.querySelector("#math-input");
  mathCorrectEl = document.querySelector("#math-correct");
  mathQuestionEl = document.querySelector("#math-question");
  rangeEl = document.querySelector("#range");
  newQuestion();
  document.querySelector("#math-form").addEventListener("submit", (e) => {
    e.preventDefault();
    submitAnswer();
  });
  document.querySelector("#range").addEventListener("change", (e) => {
    newQuestion();
  });
});