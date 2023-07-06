export function getParams() {
    return `
<div class="form-check">
    <input class="form-check-input" type="checkbox" value="" id="auth">
    <label class="form-check-label" for="auth">
    Use authentication
    </label>
</div>
<div class="input-group mb-1">
    <input type="number" min="0" max="1000" class="form-control" placeholder="Amount to enter the lottery" aria-label="Amount to enter the lottery" aria-describedby="basic-addon1">
</div>
<div class="input-group mb-1">
    <input type="number" min="0" max="1000" class="form-control" placeholder="Max number of players" aria-label="Max number of players" aria-describedby="basic-addon1">
</div>
<div class="input-group mb-1">
    <input type="number" min="0" max="1000" class="form-control" placeholder="Max number of winners" aria-label="Max number of winners" aria-describedby="basic-addon1">
</div>`;
}