import React, { useEffect, useRef, useState } from 'react';
import init, { SimulationEngine } from './wasm/blr_traffic_sim_core.js';

export default function App() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [fps, setFps] = useState<number>(0);
  const [agentCount, setAgentCount] = useState<number>(0);
  const [eastboundGreen, setEastboundGreen] = useState<boolean>(true);
  const [westboundGreen, setWestboundGreen] = useState<boolean>(false);

  useEffect(() => {
    let animationFrameId: number;
    let engine: SimulationEngine | null = null;

    async function runSimulation() {
      const wasmModule = await init();
      engine = new SimulationEngine();
      setAgentCount(engine.get_agents_count());

      const canvas = canvasRef.current;
      if (!canvas) return;
      const ctx = canvas.getContext('2d');
      if (!ctx) return;

      let lastTime = performance.now();
      let frameCount = 0;
      let lastFpsUpdate = lastTime;

      const render = (now: number) => {
        const dt = (now - lastTime) / 1000;
        lastTime = now;

        frameCount++;
        if (now - lastFpsUpdate >= 1000) {
          setFps(Math.round((frameCount * 1000) / (now - lastFpsUpdate)));
          frameCount = 0;
          lastFpsUpdate = now;
        }

        let eGreen = true;
        let wGreen = false;

        if (engine) {
          engine.step(dt);
          eGreen = engine.get_eastbound_light();
          wGreen = engine.get_westbound_light();
          
          setEastboundGreen(eGreen);
          setWestboundGreen(wGreen);
        }

        // 1. Clear background
        ctx.fillStyle = '#090d16';
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        const cx = canvas.width / 2;
        const cy = canvas.height / 2;
        const roadWidth = 120;

        // 2. Draw 4-Way Intersection Layout (Horizontal & Perpendicular Roads)
        ctx.fillStyle = '#1e293b';
        ctx.fillRect(0, cy - roadWidth / 2, canvas.width, roadWidth); // Horizontal Road
        ctx.fillRect(cx - roadWidth / 2, 0, roadWidth, canvas.height); // Perpendicular Road

        // 3. Concrete Median Dividers
        ctx.fillStyle = '#475569';
        ctx.fillRect(0, cy - 6, canvas.width, 12);
        ctx.fillRect(cx - 6, 0, 12, canvas.height);

        ctx.strokeStyle = '#eab308';
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.moveTo(0, cy); ctx.lineTo(canvas.width, cy);
        ctx.moveTo(cx, 0); ctx.lineTo(cx, canvas.height);
        ctx.stroke();

        // 4. Stop Lines on Horizontal Road
        const eastStopX = -35 * 1.2 + cx;
        const westStopX = 35 * 1.2 + cx;

        ctx.strokeStyle = '#ef4444';
        ctx.lineWidth = 3;
        ctx.beginPath();
        // Eastbound stop line is on the bottom half (left-side driving)
        ctx.moveTo(eastStopX, cy + 6);
        ctx.lineTo(eastStopX, cy + roadWidth / 2);
        // Westbound stop line is on the top half (left-side driving)
        ctx.moveTo(westStopX, cy - roadWidth / 2);
        ctx.lineTo(westStopX, cy - 6);
        ctx.stroke();

        // 5. Render Synchronized Traffic Lights for Left-Side Lanes
        // Eastbound Light (Bottom side of horizontal road)
        ctx.fillStyle = '#0f172a';
        ctx.fillRect(eastStopX - 12, cy + roadWidth / 2 + 5, 24, 30);
        ctx.strokeStyle = '#64748b';
        ctx.lineWidth = 2;
        ctx.strokeRect(eastStopX - 12, cy + roadWidth / 2 + 5, 24, 30);
        
        ctx.fillStyle = eGreen ? '#22c55e' : '#ef4444';
        ctx.beginPath();
        ctx.arc(eastStopX, cy + roadWidth / 2 + 20, 8, 0, Math.PI * 2);
        ctx.fill();

        // Westbound Light (Top side of horizontal road)
        ctx.fillStyle = '#0f172a';
        ctx.fillRect(westStopX - 12, cy - roadWidth / 2 - 35, 24, 30);
        ctx.strokeStyle = '#64748b';
        ctx.lineWidth = 2;
        ctx.strokeRect(westStopX - 12, cy - roadWidth / 2 - 35, 24, 30);
        
        ctx.fillStyle = wGreen ? '#22c55e' : '#ef4444';
        ctx.beginPath();
        ctx.arc(westStopX, cy - roadWidth / 2 - 20, 8, 0, Math.PI * 2);
        ctx.fill();

        // 6. Render Agents from WASM memory buffer
        if (engine) {
          const ptr = engine.get_agents_buffer_ptr();
          const count = engine.get_agents_count();
          const memory = wasmModule.memory;
          const buffer = new Float32Array(memory.buffer, ptr, count * 4);

          for (let i = 0; i < count; i++) {
            const x = buffer[i * 4] * 1.2 + cx;
            const y = buffer[i * 4 + 1] * 1.2 + cy;
            const classType = buffer[i * 4 + 3];

            ctx.fillStyle = classType === 0 ? '#38bdf8' : classType === 3 ? '#f43f5e' : '#fbbf24';
            ctx.fillRect(x - 6, y - 3, 12, 6);
          }
        }

        animationFrameId = requestAnimationFrame(render);
      };

      animationFrameId = requestAnimationFrame(render);
    }

    runSimulation();

    return () => {
      cancelAnimationFrame(animationFrameId);
    };
  }, []);

  return (
    <div style={{ backgroundColor: '#030712', color: '#f8fafc', minHeight: '100vh', padding: '30px', fontFamily: 'sans-serif' }}>
      <header style={{ marginBottom: '20px' }}>
        <h1 style={{ fontSize: '24px', fontWeight: 'bold', marginBottom: '8px' }}>
          4-Way Junction Layout — Indian Left-Side Driving & Synchronized Signals
        </h1>
        <p style={{ color: '#94a3b8', fontSize: '14px' }}>
          Geographic Anchor: <code>12.920566, 77.665225</code> | Left-Hand Traffic Enforced
        </p>
      </header>

      <div style={{ display: 'flex', gap: '20px', marginBottom: '20px', background: '#1e293b', padding: '12px 20px', borderRadius: '8px', width: 'fit-content', alignItems: 'center', flexWrap: 'wrap' }}>
        <div>FPS: <strong style={{ color: '#4ade80' }}>{fps}</strong></div>
        <div>Active Agents: <strong style={{ color: '#38bdf8' }}>{agentCount}</strong></div>
        <div>
          Eastbound Lane Signal: <span style={{ padding: '4px 10px', borderRadius: '4px', backgroundColor: eastboundGreen ? '#15803d' : '#b91c1c', color: '#fff', fontWeight: 'bold' }}>
            {eastboundGreen ? 'GREEN' : 'RED'}
          </span>
        </div>
        <div>
          Westbound Lane Signal: <span style={{ padding: '4px 10px', borderRadius: '4px', backgroundColor: westboundGreen ? '#15803d' : '#b91c1c', color: '#fff', fontWeight: 'bold' }}>
            {westboundGreen ? 'GREEN' : 'RED'}
          </span>
        </div>
      </div>

      <canvas 
        ref={canvasRef} 
        width={1000} 
        height={500} 
        style={{ border: '1px solid #334155', borderRadius: '12px', background: '#090d16', boxShadow: '0 10px 25px -5px rgba(0, 0, 0, 0.5)' }} 
      />
    </div>
  );
}