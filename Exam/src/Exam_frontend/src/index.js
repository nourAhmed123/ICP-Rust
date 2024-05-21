import { Exam_backend } from '../../declarations/Exam_backend';
// Optionally fetch root key for the local replica (needed for development purposes)
if (process.env.NODE_ENV !== "production") {
    agent.fetchRootKey().catch(err => {
      console.warn("Unable to fetch root key. Check to ensure that your local replica is running");
      console.error(err);
    });
  }
  

document.addEventListener('DOMContentLoaded', (event) => {
  const nat8MaxValue = 255;

  document.getElementById('insertExam').addEventListener('click', async () => {
    const examId = document.getElementById('examId').value;
    const outOf = document.getElementById('outOf').value;
    const course = document.getElementById('course').value;
    const curve = document.getElementById('curve').value;

    if (Number(outOf) > nat8MaxValue || Number(curve) > nat8MaxValue) {
      alert(`Values for 'out_of' and 'curve' must be between 0 and ${nat8MaxValue}`);
      return;
    }

    const exam = { out_of: Number(outOf), course, curve: Number(curve) };
    console.log('Inserting Exam:', exam);
    try {
      const result = await Exam_backend.insert_exam(BigInt(examId), exam);
      console.log('Insert Exam Result:', result);
      document.getElementById('examResult').innerText = JSON.stringify(result);
    } catch (err) {
      console.error('Failed to insert exam:', err);
    }
  });

  document.getElementById('fetchExam').addEventListener('click', async () => {
    const examId = document.getElementById('fetchExamId').value;
    console.log('Fetching Exam with ID:', examId);
    try {
      const result = await Exam_backend.get_exam(BigInt(examId));
      console.log('Fetch Exam Result:', result);
      document.getElementById('examResult').innerText = JSON.stringify(result);
    } catch (err) {
      console.error('Failed to fetch exam:', err);
    }
  });

  document.getElementById('insertParticipation').addEventListener('click', async () => {
    const participationId = document.getElementById('participationId').value;
    const participation = document.getElementById('participation').value;

    console.log('Inserting Participation:', { id: participationId, value: participation });
    try {
      const result = await Exam_backend.insert_participation(BigInt(participationId), BigInt(participation));
      console.log('Insert Participation Result:', result);
      document.getElementById('participationResult').innerText = `Previous value: ${result ? result.toString() : 'None'}`;
    } catch (err) {
      console.error('Failed to insert participation:', err);
    }
  });

  document.getElementById('fetchParticipation').addEventListener('click', async () => {
    const participationId = document.getElementById('fetchParticipationId').value;
    console.log('Fetching Participation with ID:', participationId);
    try {
      const result = await Exam_backend.get_participation(BigInt(participationId));
      console.log('Fetch Participation Result:', result);
      document.getElementById('participationResult').innerText = `Fetched value: ${result ? result.toString() : 'None'}`;
    } catch (err) {
      console.error('Failed to fetch participation:', err);
    }
  });
});
