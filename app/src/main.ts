import "./style.css";

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
    <h1>Nebula Frontend</h1>
    <div class="card">
      <button onclick="alert('It works!');" type="button">Click Me</button>
    </div>
  </div>
`;
