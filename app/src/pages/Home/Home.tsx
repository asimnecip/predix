import React from 'react';
import { useWallet } from '@solana/wallet-adapter-react';

const Home: React.FC = () => {
    const { connected } = useWallet();

    if (connected) {
        return (
            <>
               CONNECTED 
            </>
        );
    } else {
        return <>NOT CONNECTED</>
    }
};

export default Home;
