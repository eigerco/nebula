import { getParams as lotteryparams } from './contractparams/lotteryparams'
import './style.css'

const toolbox = document.querySelector<HTMLDivElement>('#toolbox')
if (toolbox != null) {
  toolbox.innerHTML = `
  <div class="input-group mb-3">
      <input type="text" class="form-control" placeholder="Name" aria-label="Name" aria-describedby="basic-addon1" value="MyContract">
  </div>
  <div class="dropdown">
      Contract: <button class="btn btn-secondary dropdown-toggle" type="button" data-bs-toggle="dropdown" aria-expanded="false">
          Lottery
      </button>
      <ul class="dropdown-menu">
          <li><a class="dropdown-item" href="#">Lottery</a></li>
      </ul>
  </div>
  <label class="settings">
      Settings:
  </label>
  <div class="contract-settings">
      ${lotteryparams()}
  </div>
  <div class="footer">
      <div class="input-group mb-3">
          <input type="text" class="form-control" placeholder="Author" aria-label="Author" aria-describedby="basic-addon1">
      </div>
      <div class="input-group mb-3">
          <input type="text" class="form-control" placeholder="License" aria-label="License" aria-describedby="basic-addon1">
      </div>
      <div class="buttons">
          <button type="button" class="btn btn-secondary"><i class="bi bi-clipboard"></i>Copy to clipboard</button>
          <button type="button" class="btn btn-secondary"><i class="bi bi-cloud-arrow-down"></i>Download</button>
          <button type="button" class="btn btn-secondary"><i class="bi bi-folder2-open"></i>Open in Pulsar</button>
      </div>
  </div>
`
}
