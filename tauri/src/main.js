const { invoke } = window.__TAURI__.tauri;
async function clock() {
  let new_state = await invoke("clock_processor", {});
  return new_state;
}


window.addEventListener("DOMContentLoaded", () => {

  let clock_button = document.querySelector("#clock-button");
  clock_button.addEventListener("click", async (e) => {
    let proc_div = document.querySelector("#proc-div");
    let new_state = clock().then((new_state) => {
      console.log(new_state);
      proc_div.innerHTML = new_state;
    });
  });
});
