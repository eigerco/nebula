import "./style.css";

document.querySelector<HTMLDivElement>("#toolbox")!.innerHTML = `    
    <div class="input-group mb-3">
        <input type="text" class="form-control" placeholder="Name" aria-label="Name" aria-describedby="basic-addon1">
    </div>
    <div class="mb-3">
        <div class="btn-group">
        <button class="btn btn-secondary dropdown-toggle" id="importContract" type="button" data-bs-toggle="dropdown" aria-expanded="false">
        Import contract
        </button>
        <ul class="dropdown-menu">
            <li><div class="form-check">
                <input class="form-check-input" type="checkbox" value="" id="contractA">
                <label class="form-check-label" for="contractA">
                Contract A
                </label>
            </div></li>
            <li><div class="form-check">
                <input class="form-check-input" type="checkbox" value="" id="contractB">
                <label class="form-check-label" for="contractB">
                Contract B
                </label>
            </div></li>
        </ul>
        </div>
    </div>
    <div class="form-check">
        <input class="form-check-input" type="checkbox" value="" id="tokenInterface">
        <label class="form-check-label" for="tokenInterface">
        Token interface
        </label>
    </div>
    <div class="input-group mb-3">
        <input type="text" class="form-control" placeholder="Symbol" aria-label="Symbol" aria-describedby="basic-addon1">
    </div>
    <div class="input-group mb-3">
        <input type="text" class="form-control" placeholder="Decimals" aria-label="Decimals" aria-describedby="basic-addon1">
    </div>
    <div class="input-group mb-3">
        <input type="text" class="form-control" placeholder="Author" aria-label="Author" aria-describedby="basic-addon1">
    </div>
    <div class="input-group mb-3">
        <input type="text" class="form-control" placeholder="License" aria-label="License" aria-describedby="basic-addon1">
    </div>
    <div class="btn-group-vertical" role="group" aria-label="Vertical button group">
        <button type="button" class="btn btn-secondary">Copy to clipboard</button>
        <button type="button" class="btn btn-secondary">Download</button>
        <button type="button" class="btn btn-secondary">Open in Pulsar</button>
    </div>

`;
