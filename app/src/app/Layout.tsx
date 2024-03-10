import React, { ReactNode, useCallback, useEffect, useState } from 'react';
import Navbar from '../components/Navbar/Navbar';
import Footer from '../components/Footer/Footer';
import { useWallet } from '@solana/wallet-adapter-react';
// Removed the useDispatch import as we won't be using Redux actions

interface LayoutProps {
    children: ReactNode;
}

const Layout: React.FC<LayoutProps> = ({ children }) => {
    const { publicKey } = useWallet();
    // useState for managing username state locally
    const [username, setUsername] = useState('');

    const getUsername = useCallback(async () => {
        if (publicKey) {
            const walletAddress = publicKey.toString();
            const checkUserUrl = `http://localhost:3001/users/check/${walletAddress}`;

            try {
                const response = await fetch(checkUserUrl, {
                    method: 'GET', // GET method to check the user
                    headers: {
                        'Content-Type': 'application/json',
                    },
                });
                const data = await response.json();

                if (data.exists) {
                    console.log('Existing username:', data.username);
                } else {
                    console.log('Generated username:', data.username);
                }

                // Update the username in the component's state instead of dispatching to Redux
                setUsername(data.username);
            } catch (error) {
                console.error('Error fetching username:', error);
            }
        }
    }, [publicKey]);
    
    useEffect(() => {
        getUsername();
    }, [getUsername]);

    return (
        <>
            <Navbar /> {/* Optionally pass username to Navbar if needed */}
            <main>{children}</main>
            <Footer />
        </>
    );
};

export default Layout;
