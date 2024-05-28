import React, { useState, useEffect } from 'react';
import { proposal2_backend } from '../../../declarations/proposal2_backend';

const ProposalForm = ({ editMode, proposalId, refreshProposals }) => {
 
  const [description, setDescription] = useState('');
  const [isActive, setIsActive] = useState(true);

  useEffect(() => {
    if (editMode && proposalId !== null) {
      const fetchProposal = async () => {
        const proposalData = await proposal2_backend.get_proposal(proposalId);
        if (proposalData && proposalData.length > 0) {
          const proposal = proposalData[0]; // Unwrap the proposal data
      
          setDescription(proposal.description);
          setIsActive(proposal.is_active);
        }
      };
      fetchProposal();
    }
  }, [editMode, proposalId]);

  const handleSubmit = async (e) => {
    e.preventDefault();
    const proposalData = {  description, is_active: isActive };
    if (editMode) {
      await proposal2_backend.edit_proposal(proposalId, proposalData);
    } else {
      const proposalCount = await proposal2_backend.get_proposal_count();
      await proposal2_backend.create_proposal(proposalCount, proposalData);
    }
    refreshProposals();
   
    setDescription('');
    setIsActive(true);
  };

  return (
    <form onSubmit={handleSubmit}>
    
      <input
        type="text"
        value={description}
        onChange={(e) => setDescription(e.target.value)}
        placeholder="Enter proposal description"
      />
      <label>
        Active:
        <input
          type="checkbox"
          checked={isActive}
          onChange={(e) => setIsActive(e.target.checked)}
        />
      </label>
      <button type="submit">{editMode ? 'Edit Proposal' : 'Create Proposal'}</button>
    </form>
  );
};

export default ProposalForm;
