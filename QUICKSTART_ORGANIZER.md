# 🚀 Quick Start Guide for Challenge Organizers

## Pre-Launch Checklist

Before announcing the challenge, complete these steps:

### 1. Create GitHub Repository
```bash
# Create a new repository on GitHub named: rust-screenrec-challenge
# Then push this code:

cd rust-screen-recording-assessment
git remote add origin https://github.com/OmegaLabs/rust-screenrec-challenge.git
git branch -M main
git push -u origin main
```

### 2. Configure Repository Settings

On GitHub, configure:
- ✅ Allow forking
- ✅ Enable issues
- ✅ Enable pull requests
- ✅ Add topic tags: `rust`, `challenge`, `bounty`, `screen-recording`, `tao`
- ✅ Add description: "1 TAO bounty challenge: Build a high-performance cross-platform screen recording CLI in Rust"

### 3. Create Issue Templates

Add these labels:
- `question` - For participant questions
- `submission` - For pull requests
- `evaluation-question` - For evaluation clarifications
- `bug` - For repo issues

### 4. Pin the README

Make sure the README is clearly visible with all challenge details.

### 5. Announcement Checklist

**Social Media Posts:**
- [ ] Twitter/X announcement
- [ ] Discord announcement (Bittensor, Rust communities)
- [ ] Reddit (r/rust, r/bittensor)
- [ ] Telegram groups

**Sample Announcement:**

```
🚀 NEW: Rust Screen Recording Challenge by Omega Labs

💰 Bounty: 1 TAO
⏰ Deadline: November 6, 2024
🎯 Challenge: Build a high-performance CLI screen recorder

Requirements:
✅ Cross-platform (macOS + Windows)
✅ 1080p @ 30fps with <30% CPU
✅ Screenshot + video recording
✅ Audio capture

Fork, build, and submit your PR!
🔗 [GitHub Link]

#Rust #Bittensor #TAO #BountyChallenge
```

### 6. Monitoring During Challenge

**Daily Tasks:**
- Check for questions in issues
- Monitor fork count
- Respond to clarification requests
- Update FAQ if needed

**Communication Guidelines:**
- Respond to questions within 24-48 hours
- Be encouraging but fair
- Don't favor any participant
- Don't give implementation hints beyond what's in docs

### 7. Evaluation Process (Nov 7-8)

1. **Collect Submissions**
   - All PRs by November 6, 11:59 PM PST
   - Create a tracking spreadsheet

2. **Initial Screening**
   - Does it compile?
   - Is demo video included?
   - Meets minimum requirements?

3. **Testing**
   - Test on macOS and Windows
   - Run performance benchmarks
   - Verify all features work

4. **Scoring**
   - Use EVALUATION.md criteria
   - Document scores for each submission
   - Calculate totals

5. **Winner Selection**
   - Announce winner on November 8
   - Transfer 1 TAO bounty
   - Thank all participants

### 8. Post-Challenge

**Winner Announcement Template:**
```
🎉 Rust Screen Recording Challenge - Winner Announced!

Congratulations to [Winner Name]!

🏆 Winning submission: [PR Link]
💰 1 TAO bounty awarded

Highlights:
- [Key achievement 1]
- [Key achievement 2]
- [Key achievement 3]

Thank you to all participants! Your submissions were impressive.

Check out the winning code: [Link]
```

**Follow-up Actions:**
- [ ] Transfer 1 TAO to winner
- [ ] Publish results blog post
- [ ] Share winning solution (with permission)
- [ ] Gather feedback for future challenges
- [ ] Archive repository or keep for reference

## Troubleshooting

**Issue: Too few submissions**
- Extend deadline by 2-3 days
- Increase promotion
- Lower complexity requirements slightly

**Issue: Too many questions**
- Add to FAQ section in README
- Host a Q&A session on Discord

**Issue: No submissions meet requirements**
- Award partial bounty to best attempt
- Run challenge again with adjusted requirements

## Budget Considerations

- **Bounty**: 1 TAO (~$X USD at current rate)
- **Time Investment**: ~10 hours for organization/evaluation
- **Marketing**: Social media posts (free)

## Future Improvements

After this challenge, consider:
- Increase bounty for more competition
- Add runner-up prizes
- Create a series of challenges
- Build a community around Omega Focus development

---

## Contact for Challenge

**Questions during challenge:**
GitHub Issues (response time: 24-48h)

**For urgent matters:**
[Your contact method]

---

Good luck running the challenge! 🎯
