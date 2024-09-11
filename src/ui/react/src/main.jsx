import React, { useState } from 'react'
import './index.css'
import {
  Command,
  CommandDialog,
  CommandInput,
  CommandList,
  CommandEmpty,
  CommandGroup,
  CommandItem,
  CommandShortcut,
  CommandSeparator,
} from "./components/ui/command"

const CommandMenu = () => {
  const [open, setOpen] = React.useState(false)
	const [commands, setCommands] = useState({ term: "", commands: []})

	function onChange(e) {
		setCommands({ term: e, commands: mme.comandr_search(e)})
		//setCommands({ term: e, commands: ["hi", "test"]})
	}

  function runCommand(cmd) {
    mme.comandr_run(cmd, [])
  }

  // Toggle the menu when ⌘K is pressed
  React.useEffect(() => {
    const down = (e) => {
      if (e.keyCode === 13 && e.ctrlKey) {
        e.preventDefault()
        setOpen((open) => !open)
      }
    }

    document.addEventListener('keydown', down)
    return () => document.removeEventListener('keydown', down)
  }, [])

  console.log("commands", commands.commands)

  return (
    <CommandDialog loop shouldFilter={false} open={open} onOpenChange={setOpen} label="Global Command Menu">
      <CommandInput value={commands.term} onValueChange={onChange} />
      <CommandList>
        {commands.commands.map(item => (
          <CommandItem onSelect={runCommand} > {item} </ CommandItem>
        ))}
      </CommandList>
    </CommandDialog>
  )
}

function comandr_ui_init() {
}

export { 
  comandr_ui_init,
  CommandMenu,
}
