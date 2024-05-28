import React, { useState } from 'react';
import ProposalForm from './components/ProposalForm';
import ProposalList from './components/ProposalList';
import './App.css';  // Make sure to create this file for styling

function App() {
  const [editMode, setEditMode] = useState(false);
  const [proposalId, setProposalId] = useState(null);
  const [refresh, setRefresh] = useState(false);

  const toggleEditMode = (id) => {
    setProposalId(id);
    setEditMode(!editMode);
  };

  const refreshProposals = () => {
    setRefresh(!refresh);
  };

  return (
    <div className="App">
      <header>
        <h1>ChainCortex Proposals</h1>
      </header>
      <main>
        <ProposalForm editMode={editMode} proposalId={proposalId} refreshProposals={refreshProposals} />
        <ProposalList toggleEditMode={toggleEditMode} refreshProposals={refreshProposals} />
      </main>
    </div>
  );
}

export default App;
