import React, { useState } from 'react';
import { useRouter } from 'next/router';
import web3 from 'web3';
import axios from 'axios';

export default function Home() {
    const router = useRouter();
    const [loading, setLoading] = useState(false);

    const handleConnect = async () => {
        try {
            setLoading(true);
            // @ts-ignore
            const accounts = await window.ethereum.enable();
            console.log(accounts[0]);

            const response = await axios.get(`http://127.0.0.1:8000/coins/${accounts[0]}`);

            console.log(response);
            setLoading(false);
        } catch (err) {
            console.log('==> ERROR: Could not fetch coins');
            console.error(err);
            setLoading(false);
        }
    };

    return (
        <div>
            <button onClick={handleConnect} disabled={loading}>
                Connect with Metamask
            </button>
        </div>
    );
}
