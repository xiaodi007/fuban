# 基于 Substrate 的 DeFi 借贷平台

## 项目概述

该项目旨在基于 Substrate 区块链框架创建一个强大的去中心化金融（DeFi）借贷平台。该平台将允许用户借贷加密货币和游戏物品。用户可以在平台上存储资产，并指定借贷价格和时间框架。借款人可以借用这些资产，并在各种游戏或其他应用中使用这些资产。

## 功能特点

- **多资产支持**：支持借贷不同类型的资产，包括加密货币和游戏物品。
- **跨游戏物品使用**：游戏物品可以在不同游戏中使用，确保互操作性和灵活性。
- **去中心化治理**：用户可以通过提出和投票决策参与平台治理。
- **利息计算**：借款人根据借款金额和借款期限支付利息。

## 模块

### 1. 资产模块

资产模块负责不同类型资产的注册、铸造和销毁。它确保平台可以安全高效地管理各种资产。

#### 接口:
```rust
pub trait AssetManager {
    type AssetId;
    type AccountId;
    type Balance;

    fn register_asset(asset_id: Self::AssetId, metadata: AssetMetadata) -> DispatchResult;
    fn get_asset(asset_id: Self::AssetId) -> Option<AssetMetadata>;
    fn mint(asset_id: Self::AssetId, to: Self::AccountId, amount: Self::Balance) -> DispatchResult;
    fn burn(asset_id: Self::AssetId, from: Self::AccountId, amount: Self::Balance) -> DispatchResult;
}
```

### 2. 借贷模块

借贷模块处理核心的借贷操作，包括存款、贷款、利率和时间框架。它允许用户存入资产以供借出，并管理贷款的条款。

#### 接口:
```rust
pub trait Lending {
    type AssetId;
    type AccountId;
    type Balance;
    type Moment;

    fn deposit(asset_id: Self::AssetId, from: Self::AccountId, amount: Self::Balance) -> DispatchResult;
    fn withdraw(asset_id: Self::AssetId, to: Self::AccountId, amount: Self::Balance) -> DispatchResult;
    fn borrow(asset_id: Self::AssetId, borrower: Self::AccountId, amount: Self::Balance, duration: Self::Moment) -> DispatchResult;
    fn repay(asset_id: Self::AssetId, borrower: Self::AccountId, amount: Self::Balance) -> DispatchResult;
    fn calculate_interest(asset_id: Self::AssetId, amount: Self::Balance, duration: Self::Moment) -> Self::Balance;
}
```

### 3. 借款模块

借款模块跟踪借用的资产，并确保适当的还款机制。它帮助借款人管理他们的贷款，跟踪借款金额和借款期限。

#### 接口:
```rust
pub trait Borrowing {
    type AssetId;
    type AccountId;
    type Balance;
    type Moment;

    fn borrow(asset_id: Self::AssetId, borrower: Self::AccountId, amount: Self::Balance, duration: Self::Moment) -> DispatchResult;
    fn repay(asset_id: Self::AssetId, borrower: Self::AccountId, amount: Self::Balance) -> DispatchResult;
    fn get_borrowed_amount(asset_id: Self::AssetId, borrower: Self::AccountId) -> Self::Balance;
    fn get_borrowing_duration(asset_id: Self::AssetId, borrower: Self::AccountId) -> Self::Moment;
}
```

### 4. 跨游戏物品使用模块

跨游戏物品使用模块确保游戏物品可以在不同游戏中使用，提高互操作性和用户体验。

#### 接口:
```rust
pub trait CrossGameItemUsage {
    type GameItemId;
    type AccountId;

    fn register_game(game_id: u32, game_metadata: GameMetadata) -> DispatchResult;
    fn get_game(game_id: u32) -> Option<GameMetadata>;
    fn transfer_item(item_id: Self::GameItemId, from_game: u32, to_game: u32, owner: Self::AccountId) -> DispatchResult;
}
```

### 5. 治理模块

治理模块允许用户参与平台的决策过程。用户可以提出变更并投票，确保去中心化控制。

#### 接口:
```rust
pub trait Governance {
    type ProposalId;
    type AccountId;
    type Balance;

    fn propose_change(proposal_id: Self::ProposalId, proposer: Self::AccountId, proposal_details: ProposalDetails) -> DispatchResult;
    fn vote(proposal_id: Self::ProposalId, voter: Self::AccountId, vote: Vote) -> DispatchResult;
    fn implement_proposal(proposal_id: Self::ProposalId) -> DispatchResult;
}
```

## 入门指南

### 先决条件

- Rust 和 Cargo
- Substrate 节点模板
- Substrate FRAME

### 安装

1. **克隆仓库**:
    ```bash
    git clone https://github.com/xiaodi007/fuban
    cd fuban
    ```

2. **构建项目**:
    ```bash
    cargo build --release
    ```

3. **运行节点**:
    ```bash
    ./target/release/node-template --dev
    ```

### 使用

- **注册资产**:
    ```rust
    // 注册资产的示例调用
    AssetManager::register_asset(asset_id, metadata);
    ```

- **存入资产**:
    ```rust
    // 存入资产的示例调用
    Lending::deposit(asset_id, from, amount);
    ```

- **借入资产**:
    ```rust
    // 借入资产的示例调用
    Borrowing::borrow(asset_id, borrower, amount, duration);
    ```

