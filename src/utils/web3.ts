import { WalletProvider, WALLET_PROVIDER_DEVNET } from "@multiversx/sdk-web-wallet-provider";
import { TransactionWatcher } from "@multiversx/sdk-core";
import { ApiNetworkProvider } from "@multiversx/sdk-network-providers";

// Define MultiversX network configurations
export const CHAIN_CONFIG = {
  devnet: {
    chainId: 'D',
    chainName: 'MultiversX Devnet',
    nativeCurrency: { name: 'eGLD', symbol: 'eGLD', decimals: 18 },
    rpcUrls: ['https://devnet-gateway.multiversx.com'],
    blockExplorerUrls: ['https://devnet-explorer.multiversx.com'],
    apiUrl: 'https://devnet-api.multiversx.com',
    walletAddress: WALLET_PROVIDER_DEVNET,
    iconPath: '/chains/multiversx.png'
  },
  testnet: {
    chainId: 'T',
    chainName: 'MultiversX Testnet',
    nativeCurrency: { name: 'eGLD', symbol: 'eGLD', decimals: 18 },
    rpcUrls: ['https://testnet-gateway.multiversx.com'],
    blockExplorerUrls: ['https://testnet-explorer.multiversx.com'],
    apiUrl: 'https://testnet-api.multiversx.com',
    walletAddress: 'https://testnet-wallet.multiversx.com',
    iconPath: '/chains/multiversx.png'
  },
  mainnet: {
    chainId: '1',
    chainName: 'MultiversX Mainnet',
    nativeCurrency: { name: 'eGLD', symbol: 'eGLD', decimals: 18 },
    rpcUrls: ['https://gateway.multiversx.com'],
    blockExplorerUrls: ['https://explorer.multiversx.com'],
    apiUrl: 'https://api.multiversx.com',
    walletAddress: 'https://wallet.multiversx.com',
    iconPath: '/chains/multiversx.png'
  }
};

export type ChainKey = keyof typeof CHAIN_CONFIG;

// MultiversX wallet provider singleton
let walletProvider: WalletProvider | null = null;

// Initialize wallet provider
export const getWalletProvider = (chainKey: ChainKey = 'devnet'): WalletProvider => {
  if (!walletProvider) {
    walletProvider = new WalletProvider(CHAIN_CONFIG[chainKey].walletAddress);
  }
  return walletProvider;
};

// Get network provider for querying the blockchain
export const getNetworkProvider = (chainKey: ChainKey = 'devnet') => {
  return new ApiNetworkProvider(CHAIN_CONFIG[chainKey].apiUrl);
};

// Connect to MultiversX wallet
export const connectWallet = async (chainKey: ChainKey = 'devnet') => {
  const provider = getWalletProvider(chainKey);
  
  // Build the callback URL (current URL without query parameters)
  const callbackUrl = encodeURIComponent(window.location.href.split('?')[0]);
  
  // Redirect to the wallet login page
  await provider.login({
    callbackUrl: callbackUrl,
  });
  
  // The actual connection happens when the wallet redirects back with address info
  return { 
    success: true,
    message: 'Redirecting to MultiversX Wallet...'
  };
};

// Get address from URL after wallet redirect
export const getAddressFromUrl = (): string | null => {
  const urlParams = new URLSearchParams(window.location.search);
  return urlParams.get('address');
};

// Sign and send a transaction
export const signTransaction = async (transaction: any, chainKey: ChainKey = 'devnet') => {
  const provider = getWalletProvider(chainKey);
  const callbackUrl = encodeURIComponent(window.location.href.split('?')[0]);
  
  // Redirect to the wallet for signing
  await provider.signTransaction(transaction, {
    callbackUrl: callbackUrl
  });
  
  return {
    success: true,
    message: 'Redirecting to MultiversX Wallet for signing...'
  };
};

// Get a signed transaction from URL after wallet redirect
export const getSignedTransactionFromUrl = () => {
  const provider = getWalletProvider();
  return provider.getTransactionsFromWalletUrl();
};

// Disconnect wallet
export const disconnectWallet = async (chainKey: ChainKey = 'devnet') => {
  const provider = getWalletProvider(chainKey);
  const callbackUrl = encodeURIComponent(window.location.href.split('?')[0]);
  
  await provider.logout({
    callbackUrl: callbackUrl
  });
  
  return {
    success: true,
    message: 'Logged out successfully'
  };
};

// Check if transaction is completed
export const waitForTransaction = async (txHash: string, chainKey: ChainKey = 'devnet') => {
  const networkProvider = getNetworkProvider(chainKey);
  const watcher = new TransactionWatcher(networkProvider);
  return await watcher.awaitCompleted(txHash);
};