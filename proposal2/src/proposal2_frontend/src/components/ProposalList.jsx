import React, { useState, useEffect } from "react";
import { proposal2_backend } from "../../../declarations/proposal2_backend";

const ProposalList = ({ toggleEditMode, refreshProposals }) => {
  const [proposals, setProposals] = useState([]);
  const [proposalCount, setProposalCount] = useState(0);

  useEffect(() => {
    const fetchProposals = async () => {
      const count = await proposal2_backend.get_proposal_count();
      setProposalCount(count);
      const allProposals = [];
      for (let i = 0; i < count; i++) {
        const proposalData = await proposal2_backend.get_proposal(i);
        if (proposalData && proposalData.length > 0) {
          const proposal = proposalData[0]; // Unwrap the proposal data
          console.log("Fetched proposal:", proposal);
          allProposals.push({ id: i, ...proposal });
        }
      }
      setProposals(allProposals);
    };

    fetchProposals();
  }, [refreshProposals]);

  const handleVote = async (id, choice) => {
    await proposal2_backend.vote(id, choice);
    refreshProposals();
  };

  const handleEndProposal = async (id) => {
    await proposal2_backend.end_proposal(id);
    refreshProposals();
  };

  return (
    <div>
      <h1>Proposals</h1>
      <ul>
        {proposals.map((proposal) => (
          <li key={proposal.id}>
            {console.log("Proposal Description:", proposal.description)}
            <h2>{proposal.name}</h2>
            <p>{proposal.description}</p>
            <p>
              Approve: {proposal.approve}{" "}
              <button
                onClick={() => handleVote(proposal.id, { Approve: null })}
              >
                Approve
              </button>
            </p>
            <p>Reject: {proposal.reject}   <button onClick={() => handleVote(proposal.id, { Reject: null })}>
              Reject
            </button></p>
            <p>Pass: {proposal.pass}       <button onClick={() => handleVote(proposal.id, { Pass: null })}>
              Pass
            </button></p>

          
      
            <button
              className="edit"
              onClick={() => toggleEditMode(proposal.id)}
            >
              Edit
            </button>
            <button
              className="end"
              onClick={() => handleEndProposal(proposal.id)}
            >
              End Proposal
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default ProposalList;
