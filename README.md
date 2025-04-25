# Basic Crowdfunding Platform Smart Contract

## Table of Contents
- [Project Title](#project-title)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Contract Details](#contract-details)

## Project Title

**Basic Crowdfunding Platform**

## Project Description

The Basic Crowdfunding Platform enables creators to launch fundraising campaigns directly on the blockchain. Backers can browse and contribute funds to active campaigns, helping bring creative, social, and tech projects to life in a transparent and decentralized manner.

## Project Vision

Our vision is to simplify project funding by eliminating intermediaries. This smart contract empowers anyone to seek financial support for their ideas and ensures that all contributions are traceable and securely stored on-chain.

## Key Features

- 🎯 **Create Campaigns**: Users can create campaigns by setting funding goals and providing descriptions.
- 🤝 **Contribute**: Contributors can fund active campaigns easily.
- 📊 **View Campaigns**: Anyone can view the status of any campaign.
- 🔒 **Goal Logic**: Campaigns auto-close once their goal is reached.

## Contract Details
### Contract Address: CA4MK7ZSOXMFA4WCCSZTGEDTAQG62HMWBYCA2EYFTZ4OZUN6X2PMNVRP

This smart contract is written using the Soroban SDK and includes the following key functionalities:

### 1. `create_campaign`
Allows users to create a new crowdfunding campaign by specifying a title, description, and funding goal.

### 2. `contribute`
Contributors can fund campaigns. The campaign is marked as completed once its goal is reached.

### 3. `get_campaign`
Fetches the current details of a campaign, including the total pledged amount and its active status.

---

This contract offers a lightweight and extensible foundation for building full-featured crowdfunding platforms on Stellar's Soroban smart contract framework.
