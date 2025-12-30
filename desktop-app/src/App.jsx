import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { save } from '@tauri-apps/api/dialog'
import { writeBinaryFile } from '@tauri-apps/api/fs'
import { appWindow } from '@tauri-apps/api/window'

function formatBytes(bytes) {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

function App() {
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [status, setStatus] = useState('idle') // idle, authenticating, downloading, success, error
  const [error, setError] = useState('')
  const [stats, setStats] = useState(null)

  const handleSubmit = async (e) => {
    e.preventDefault()
    setError('')
    setStats(null)
    
    if (!email || !password) {
      setError('Please enter your email and password')
      return
    }

    try {
      // Step 1: Authenticate
      setStatus('authenticating')
      const authResult = await invoke('authenticate', { email, password })
      
      if (!authResult.success) {
        throw new Error(authResult.error || 'Authentication failed')
      }

      // Step 2: Download workouts
      setStatus('downloading')
      const downloadResult = await invoke('download_workouts', {
        idToken: authResult.id_token,
        userId: authResult.user_id
      })

      if (!downloadResult.success) {
        throw new Error(downloadResult.error || 'Download failed')
      }

      // Step 3: Ask where to save (default to .json.gz)
      const today = new Date().toISOString().slice(0, 10)
      const filePath = await save({
        defaultPath: `tonal_workouts_${today}.json.gz`,
        filters: [
          { name: 'Compressed JSON', extensions: ['json.gz', 'gz'] },
          { name: 'JSON', extensions: ['json'] }
        ]
      })

      if (filePath) {
        // Determine if we should save compressed or uncompressed
        const isGzip = filePath.endsWith('.gz')
        
        if (isGzip && downloadResult.gzip_data) {
          // Save compressed version
          await writeBinaryFile(filePath, new Uint8Array(downloadResult.gzip_data))
        } else if (downloadResult.json_data) {
          // Save uncompressed JSON
          const encoder = new TextEncoder()
          await writeBinaryFile(filePath, encoder.encode(downloadResult.json_data))
        }
        
        setStats({
          workouts: downloadResult.stats?.workouts || 0,
          customWorkouts: downloadResult.stats?.custom_workouts || 0,
          volume: downloadResult.stats?.total_volume || 0,
          jsonSize: downloadResult.stats?.json_size || 0,
          gzipSize: downloadResult.stats?.gzip_size || 0,
          compressionRatio: downloadResult.stats?.compression_ratio || 0,
          filePath,
          isGzip
        })
        setStatus('success')
      } else {
        // User cancelled save dialog
        setStatus('idle')
      }

    } catch (err) {
      setError(err.message || 'Something went wrong')
      setStatus('error')
    }
  }

  const reset = () => {
    setStatus('idle')
    setError('')
    setStats(null)
    setPassword('')
  }

  return (
    <div className="min-h-screen flex items-center justify-center p-8">
      <div className="w-full max-w-md">
        {/* Logo/Header */}
        <div className="text-center mb-8">
          <div className="text-5xl mb-3">🏋️</div>
          <h1 className="text-3xl font-bold text-white mb-2">ToneGet</h1>
          <p className="text-gray-400">Enter your Tonal credentials to export your workout data</p>
        </div>

        {/* Main Card */}
        <div className="bg-white/10 backdrop-blur-lg rounded-2xl p-8 shadow-2xl border border-white/20">
          
          {status === 'idle' || status === 'error' ? (
            <form onSubmit={handleSubmit}>
              <div className="mb-5">
                <label className="block text-sm font-medium text-gray-300 mb-2">
                  Tonal Email
                </label>
                <input
                  type="email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  className="w-full px-4 py-3 bg-white/10 border border-white/20 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition"
                  placeholder="you@example.com"
                />
              </div>

              <div className="mb-6">
                <label className="block text-sm font-medium text-gray-300 mb-2">
                  Password
                </label>
                <input
                  type="password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  className="w-full px-4 py-3 bg-white/10 border border-white/20 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition"
                  placeholder="••••••••"
                />
              </div>

              {error && (
                <div className="mb-4 p-3 bg-red-500/20 border border-red-500/50 rounded-lg text-red-200 text-sm">
                  {error}
                </div>
              )}

              <button
                type="submit"
                className="w-full py-4 bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-500 hover:to-purple-500 text-white font-semibold rounded-lg transition transform hover:scale-[1.02] active:scale-[0.98] shadow-lg"
              >
                Download My Data
              </button>
            </form>
          ) : status === 'authenticating' ? (
            <div className="text-center py-8">
              <div className="animate-spin text-4xl mb-4">🔐</div>
              <p className="text-lg text-white">Signing in to Tonal...</p>
              <p className="text-sm text-gray-400 mt-2">This may take a moment</p>
            </div>
          ) : status === 'downloading' ? (
            <div className="text-center py-8">
              <div className="flex justify-center mb-4">
                <div className="w-12 h-12 border-4 border-white/20 border-t-blue-500 rounded-full animate-spin"></div>
              </div>
              <p className="text-lg text-white">Downloading workouts...</p>
              <p className="text-sm text-gray-400 mt-2">This may take a minute</p>
            </div>
          ) : status === 'success' ? (
            <div className="text-center py-6">
              <div className="text-5xl mb-4">🎉</div>
              <h2 className="text-2xl font-bold text-white mb-2">Export Complete!</h2>
              
              {stats && (
                <div className="bg-white/10 rounded-lg p-4 my-6 text-left">
                  <div className="flex justify-between py-2 border-b border-white/10">
                    <span className="text-gray-400">Workouts</span>
                    <span className="text-white font-semibold">{stats.workouts.toLocaleString()}</span>
                  </div>
                  {stats.customWorkouts > 0 && (
                    <div className="flex justify-between py-2 border-b border-white/10">
                      <span className="text-gray-400">Custom Workouts</span>
                      <span className="text-white font-semibold">{stats.customWorkouts}</span>
                    </div>
                  )}
                  <div className="flex justify-between py-2 border-b border-white/10">
                    <span className="text-gray-400">Total Volume</span>
                    <span className="text-white font-semibold">{stats.volume.toLocaleString()} lbs</span>
                  </div>
                  <div className="flex justify-between py-2 border-b border-white/10">
                    <span className="text-gray-400">File Size</span>
                    <span className="text-white font-semibold">
                      {formatBytes(stats.isGzip ? stats.gzipSize : stats.jsonSize)}
                    </span>
                  </div>
                  {stats.isGzip && stats.compressionRatio > 0 && (
                    <div className="flex justify-between py-2">
                      <span className="text-gray-400">Compression</span>
                      <span className="text-emerald-400 font-semibold">
                        {stats.compressionRatio.toFixed(0)}% smaller
                      </span>
                    </div>
                  )}
                </div>
              )}
              
              <p className="text-gray-400 text-sm mb-6">
                Your data has been saved. Upload it to the Overtone dashboard!
              </p>
              
              <div className="flex gap-3">
                <button
                  onClick={reset}
                  className="flex-1 px-4 py-3 bg-white/10 hover:bg-white/20 text-white rounded-lg transition"
                >
                  Export Again
                </button>
                <button
                  onClick={() => appWindow.close()}
                  className="flex-1 px-4 py-3 bg-white/20 hover:bg-white/30 text-white rounded-lg transition"
                >
                  Close
                </button>
              </div>
            </div>
          ) : null}
        </div>

        {/* Footer */}
        <div className="mt-6 text-center">
          <p className="text-gray-500 text-xs">
            Not affiliated with Tonal Systems, Inc.
          </p>
          <p className="text-gray-600 text-xs mt-1">
            Your credentials are sent directly to Tonal and never stored.
          </p>
        </div>
      </div>
    </div>
  )
}

export default App
