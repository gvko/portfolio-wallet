import React, { useState } from 'react';
import { useRouter } from 'next/router';
import web3 from 'web3';
import axios from 'axios';

export default function Home() {
    const router = useRouter();
    const [loading, setLoading] = useState(false);
    const [tokens, setTokens] = useState([]);

    const handleConnect = async () => {
        try {
            setLoading(true);
            // @ts-ignore
            const accounts = await window.ethereum.enable();
            console.log(accounts[0]);

            const response = await axios.get(`http://127.0.0.1:8000/coins/${accounts[0]}`);

            setTokens(response.data);
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
            {tokens.length > 0 && (
                <table>
                    <thead>
                    <tr>
                        <th>Symbol</th>
                        <th>Name</th>
                        <th>Balance</th>
                        <th>Logo</th>
                    </tr>
                    </thead>
                    <tbody>
                    {tokens.map(token => (
                        <tr key={token.symbol}>
                            <td>{token.symbol}</td>
                            <td>{token.name}</td>
                            <td>{token.balance}</td>
                            <td>
                                <img src={token.logo} alt={token.name}/>
                            </td>
                        </tr>
                    ))}
                    </tbody>
                </table>
            )}
        </div>
    );
}
