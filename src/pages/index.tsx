import {invoke} from "@tauri-apps/api/tauri";

function App() {
  const executeCommands = () => {
    invoke('command_with_message', { message: 'some message' }).then(message => {
      console.log('command_with_message', message)
    })
  }

  return (
    <div className="container">
      <p>
        Hello world
      </p>

      <div className="row">
        <button onClick={executeCommands}>Click to execute command</button>
      </div>
    </div>
  );
}

export default App;
