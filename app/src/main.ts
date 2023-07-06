import './style.css'

const app = document.querySelector<HTMLDivElement>('#app')
if (app != null) {
  app.innerHTML = `
  <div>
    <h1>Nebula Frontend</h1>
    <div class="card">
      <button onclick="alert('It works!');" type="button">Click Me</button>
    </div>
  </div>
`
}
