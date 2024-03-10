import React, { useEffect, useCallback, useState, useRef } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { useNavigate } from 'react-router-dom';

// Removed the useSelector import as we won't be using Redux

const WalletButton: React.FC = () => {
    const { connected, disconnect } = useWallet();
    const navigate = useNavigate();

    // Removed the useSelector call and replaced it with useState
    const [username, setUsername] = useState<string>(""); // Initialize username state here, possibly with a default or from props/context

    const [isOpen, setIsOpen] = useState<boolean>(false);
    const toggleContainer = useRef<HTMLDivElement | null>(null);

    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            if (toggleContainer.current && !toggleContainer.current.contains(event.target as Node)) {
                setIsOpen(false);
            }
        };

        window.addEventListener('click', handleClickOutside);
        return () => {
            window.removeEventListener('click', handleClickOutside);
        };
    }, []);

    const handleProfile = () => {
        setIsOpen(false);
        navigate("/profile");
    }

    const handleLogout = async () => {
        setIsOpen(false);
        /* await disconnect(); */
        navigate("/login");
    }

    if (connected) {
        return (
            <div ref={toggleContainer} style={{ position: 'relative', display: 'inline-block' }}>
                <button onClick={() => setIsOpen(!isOpen)}>{username}</button>
                {isOpen && (
                    <div style={{ position: 'absolute', backgroundColor:'#121212', marginTop: '2px' }}>
                        <button style={{ display: 'block', width: '100%', padding: '10px', textAlign: 'left' }} onClick={handleProfile}>Profile</button>
                        <button style={{ display: 'block', width: '100%', padding: '10px', textAlign: 'left' }} onClick={handleLogout}>Logout</button>
                    </div>
                )}
            </div>
        )
    } else {
        return (
            <button onClick={() => navigate("/login")}>Sign In</button> 
        )
    }
};

export default WalletButton;
