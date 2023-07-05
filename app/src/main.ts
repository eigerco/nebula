import "./style.css";

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div id="toolbox"></div>
  <script type="module" src="/src/toolbox.ts"></script>

  <div class="editor">
    <h1>Nebula Frontend</h1>    
    <div class="card">
      <button onclick="alert('It works!');" type="button">Click Me</button>
    </div>
  </div>
`;
