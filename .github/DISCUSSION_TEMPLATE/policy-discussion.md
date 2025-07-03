---
name: "Policy Discussion"
about: "Start a discussion on governance policies or protocol design for a .NET-based MeshChain"
---

# Policy Discussion: [Topic]

## Background
- MeshChain client built with **.NET MAUI** or **Xamarin.Forms** for lightweight wallet functionality.
- **ASP.NET Core** used for Sync Agent and **LibP2P.Csharp** for mesh networking implementation.
- Objective: Evaluate requirements and benefits of a fully .NET-based solution.

## Goals
- Define standards for using **LibP2P.Csharp** (discovery, gossip protocols).
- Specify .NET libraries and patterns for **BLE** and **Wi-Fi Direct** communication.
- Design smart contracts using **Nethereum** or **Stratis** for stablecoin issuance and yield distribution.

## Open Questions
- **Lightweight Client:** Which framework is best: .NET MAUI or Xamarin.Forms?
- **Mesh Networking:** Which .NET BLE/Wi-Fi Direct package should we adopt?
- **Merkle Proofs:** How should Merkle tree and proof generation be implemented in C#?

## Proposed Approaches
- **Approach A:** Utilize **LibP2P.Csharp** with default configuration for peer discovery and gossip.
- **Approach B:** Implement a custom mesh network using **System.Net.Sockets** and **UDP Broadcast**.
- **Approach C:** Combine **Plugin.BLE** and **Xamarin.WiFi** for mesh messaging between mobile devices.

## Next Steps
1. Consolidate feedback and select the preferred approach.
2. Assign .NET team members to implement the networking module.
3. Create a PR with sample code demonstrating mesh connectivity and transaction propagation in a test environment.
