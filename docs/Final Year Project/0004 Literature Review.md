---
date-created: 2025-01-10
date-modified: 2025-01-11
tags:
  - Documentation
  - FYP
cssclasses:
---
## Generative Agents: Interactive Simulacra of Human Behaviour

### Main Objective and Research Questions
The aim of the study was to develop generative agents which can simulate believable human behaviour. Some of the key research questions asked in this papers includes
1. How can generative agents simulate realistic individual and social behaviours over time?
2. What architecture is required to ensure agents maintain long-term coherence in memory, behaviour, and interaction?
3. How can emergent social dynamic arise from interactions between generative agents?

### Methods Used (Experiments, Modelling and Frameworks)

The researchers created a sandbox environment, which is inspired by the game "The Sims" which is populated with 25 generative agents. 

**Agent Architecture**
The researchers integrated a memory stream framework for storing experiences, reflection mechanisms for higher-order generalizations and a dynamic planning system. 

**Implementation**
They used ChatGPT to enable agents to perceive their environment, retrieve memories and synthesize plans and engage in various interactions. 

**Evaluation**
The researchers have tested agents ability to maintain self-knowledge, retrieve memories, generate plans, react and reflect in isolation. They have also observed emergent social dynamics over two simulated days. 

**Interactive Features**
The researchers have also designed agents to respond to user commands, adapt to environmental changes as well as demonstrate evolving relationships and behaviours.

### Key Findings and Conclusions
**Agent Coherence**
The architecture integration of observation, reflection and planning proved essential for generating believable behaviour and interactions. Removing the framework would significantly reduce agent performance. 

**Emergent Dynamics**
Social phenomena, such as information diffusion and relationship formation arose organically from agent interactions. For example when a single agent was instructed to host a Valentine's Day event, it lead to a town wide party. As information about valentines day spread.

**Limitations**
Some challenges  that occurred was the failure of memory retrieval, fabrication of events and occasional unnatural conversational styles due to the language model. 

**Applications and Implications**
Some applications span gaming, training simulations and prototyping social systems. Some ethical issues can be considered such as mitigating parasocial  relationships and addressing the risk of misuse. 

## MemoRAG: Moving towards Next-Gen RAG Via Memory-Inspired Knowledge Discovery

### Main Objective and Research Questions
This study introduces a new concept called MemoRAG, an advanced Retrieval-Augmented Generation (RAG) framework aimed at overcoming limitations of traditional RAG systems. 

The Key Research objectives
1. How can a memory-inspired approach improve retrieval and generation for ambiguous or complex tasks?
2. Can long-context tasks involve implicit queries and distributed evidence be handled effectively using global memory mechanisms.
3. What architecture and training paradigms optimize the balance between memory efficiency and task-specific accuracy?

### Methods Used (Experiments, Modelling and Frameworks)
The implementation of MemoRAG involves a dual architecture
- A light LLM for global memory creation over extended contexts.
- A heavy LLM for retrieval augmented generation tasks.

As well as a memory module which has progressive compression of input tokens into a compact memory token while preserving semantic integrity.

Pre-trained on large datasets such as RedPajama to form memory structures. 

They evaluated the models using benchmarks such as UltraDomain, this includes complex tasks from diverse domains such as law, finance and education.
### Key Findings and Conclusions

**Enhanced Retrieval and Generation**
MemoRAG significantly outperformed RAG systems, especially in tasks which require implicit reasoning or long-context integration.

**Applicability Across Domains**
Showcased robust performance across various domains.

**Challenges and Future Directions**
While MemoRAG excels in many areas. Some challenges include scalability with extremely high context lengths.

## Meta-Control of Dialogue Systems Using Large Language Models

### Main Objective and Research Questions

### Methods Used (Experiments, Modelling and Frameworks)

### Key Findings and Conclusions

## Comparative Analysis
### Similarities
### Differences
### Strengths and Weaknesses
### Relevance
## References

\[1] J. S. Park, J. C. O’Brien, C. J. Cai, M. R. Morris, P. Liang, and M. S. Bernstein, “Generative Agents: Interactive Simulacra of Human Behavior,” arXiv:2304.03442 \[cs], Apr. 2023, Available: https://arxiv.org/abs/2304.03442

\[2] H. Qian, P. Zhang, Z. Liu, K. Mao, and Z. Dou, “MemoRAG: Moving towards Next-Gen RAG Via Memory-Inspired Knowledge Discovery,” _arXiv.org_, 2024. https://arxiv.org/abs/2409.05591

\[3] K. Shukuri _et al._, “Meta-control of Dialogue Systems Using Large Language Models,” _arXiv.org_, 2023. https://arxiv.org/abs/2312.13715