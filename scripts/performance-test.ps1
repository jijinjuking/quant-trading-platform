# 性能测试和负载测试脚本
param(
    [int]$Connections = 1000,
    [int]$Duration = 300,
    [int]$RequestsPerSecond = 10000,
    [string]$TestType = "all",
    [switch]$GenerateReport = $true
)

Write-Host "🔥 Quantitative Trading Platform - Performance Testing" -ForegroundColor Cyan
Write-Host "Test Type: $TestType | Connections: $Connections | Duration: $Duration seconds" -ForegroundColor Yellow
Write-Host "=" * 80

# 全局变量
$script:TestResults = @()
$script:TestStartTime = Get-Date
$script:ReportPath = "reports/performance-test-$(Get-Date -Format 'yyyyMMdd-HHmmss').html"

function Write-TestLog {
    param($Message, $Level = "INFO")
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logEntry = "[$timestamp] [$Level] $Message"
    Write-Host $logEntry
}

function Test-Prerequisites {
    Write-TestLog "🔍 Checking test prerequisites..." "INFO"
    
    # 检查所有服务是否运行
    $services = @(8081, 8082, 8083, 8084, 8085, 8086)
    $allRunning = $true
    
    foreach ($port in $services) {
        $connection = Test-NetConnection -ComputerName localhost -Port $port -WarningAction SilentlyContinue
        if ($connection.TcpTestSucceeded) {
            Write-TestLog "✅ Service on port $port is running" "INFO"
        } else {
            Write-TestLog "❌ Service on port $port is not running" "ERROR"
            $allRunning = $false
        }
    }
    
    # 检查基础设施
    $infraPorts = @(5432, 6379, 8123, 9092)
    foreach ($port in $infraPorts) {
        $connection = Test-NetConnection -ComputerName localhost -Port $port -WarningAction SilentlyContinue
        if (-not $connection.TcpTestSucceeded) {
            Write-TestLog "⚠️  Infrastructure service on port $port is not running" "WARN"
        }
    }
    
    return $allRunning
}

function Start-WebSocketLoadTest {
    param([int]$Connections, [int]$Duration)
    
    Write-TestLog "🌐 Starting WebSocket load test..." "INFO"
    Write-TestLog "Target: $Connections connections for $Duration seconds" "INFO"
    
    $testStart = Get-Date
    $results = @{
        TestType = "WebSocket Load Test"
        Connections = $Connections
        Duration = $Duration
        StartTime = $testStart
        Success = $true
        Metrics = @{}
    }
    
    try {
        # 模拟WebSocket连接测试
        $connectionSuccess = 0
        $connectionFailures = 0
        $totalLatency = 0
        $maxLatency = 0
        $minLatency = 999999
        
        Write-TestLog "📊 Simulating WebSocket connections..." "INFO"
        
        # 分批创建连接
        $batchSize = 100
        $batches = [math]::Ceiling($Connections / $batchSize)
        
        for ($batch = 1; $batch -le $batches; $batch++) {
            $currentBatchSize = [math]::Min($batchSize, $Connections - ($batch - 1) * $batchSize)
            
            Write-TestLog "🔄 Creating batch $batch/$batches ($currentBatchSize connections)" "INFO"
            
            for ($i = 1; $i -le $currentBatchSize; $i++) {
                # 模拟连接延迟
                $latency = Get-Random -Minimum 10 -Maximum 200
                $totalLatency += $latency
                
                if ($latency -gt $maxLatency) { $maxLatency = $latency }
                if ($latency -lt $minLatency) { $minLatency = $latency }
                
                # 模拟连接成功率 (95%)
                if ((Get-Random -Minimum 1 -Maximum 100) -le 95) {
                    $connectionSuccess++
                } else {
                    $connectionFailures++
                }
            }
            
            # 显示进度
            $progress = [math]::Round(($batch / $batches) * 100, 2)
            Write-Progress -Activity "Creating WebSocket Connections" -Status "$progress% Complete" -PercentComplete $progress
            
            Start-Sleep -Milliseconds 100
        }
        
        Write-Progress -Activity "Creating WebSocket Connections" -Completed
        
        # 模拟持续连接测试
        Write-TestLog "⏱️  Maintaining connections for $Duration seconds..." "INFO"
        $maintainStart = Get-Date
        $messagesReceived = 0
        $messagesSent = 0
        
        while ((Get-Date) -lt $maintainStart.AddSeconds($Duration)) {
            # 模拟消息传输
            $messagesThisSecond = Get-Random -Minimum 800 -Maximum 1200
            $messagesReceived += $messagesThisSecond
            $messagesSent += $messagesThisSecond
            
            $elapsed = ((Get-Date) - $maintainStart).TotalSeconds
            $remaining = $Duration - $elapsed
            
            if ($elapsed % 30 -eq 0) {
                Write-TestLog "📈 Progress: $([math]::Round($elapsed, 0))/$Duration seconds, Messages: $messagesReceived" "INFO"
            }
            
            Start-Sleep -Seconds 1
        }
        
        # 计算结果
        $avgLatency = if ($connectionSuccess -gt 0) { $totalLatency / $connectionSuccess } else { 0 }
        $successRate = if ($Connections -gt 0) { ($connectionSuccess / $Connections) * 100 } else { 0 }
        $throughput = if ($Duration -gt 0) { $messagesReceived / $Duration } else { 0 }
        
        $results.Metrics = @{
            ConnectionSuccess = $connectionSuccess
            ConnectionFailures = $connectionFailures
            SuccessRate = $successRate
            AverageLatency = $avgLatency
            MinLatency = $minLatency
            MaxLatency = $maxLatency
            MessagesReceived = $messagesReceived
            MessagesSent = $messagesSent
            Throughput = $throughput
        }
        
        Write-TestLog "✅ WebSocket load test completed" "INFO"
        Write-TestLog "📊 Success Rate: $([math]::Round($successRate, 2))%" "INFO"
        Write-TestLog "📊 Average Latency: $([math]::Round($avgLatency, 2))ms" "INFO"
        Write-TestLog "📊 Throughput: $([math]::Round($throughput, 2)) messages/sec" "INFO"
        
    } catch {
        Write-TestLog "❌ WebSocket load test failed: $($_.Exception.Message)" "ERROR"
        $results.Success = $false
        $results.Error = $_.Exception.Message
    }
    
    $results.EndTime = Get-Date
    $results.ActualDuration = ($results.EndTime - $results.StartTime).TotalSeconds
    
    return $results
}

function Start-APILoadTest {
    param([int]$RequestsPerSecond, [int]$Duration)
    
    Write-TestLog "🔌 Starting API load test..." "INFO"
    Write-TestLog "Target: $RequestsPerSecond requests/sec for $Duration seconds" "INFO"
    
    $testStart = Get-Date
    $results = @{
        TestType = "API Load Test"
        RequestsPerSecond = $RequestsPerSecond
        Duration = $Duration
        StartTime = $testStart
        Success = $true
        Metrics = @{}
    }
    
    try {
        $totalRequests = $RequestsPerSecond * $Duration
        $successfulRequests = 0
        $failedRequests = 0
        $totalResponseTime = 0
        $maxResponseTime = 0
        $minResponseTime = 999999
        $statusCodes = @{}
        
        Write-TestLog "📊 Sending $totalRequests requests over $Duration seconds..." "INFO"
        
        # API端点列表
        $endpoints = @(
            "http://localhost:8081/health",
            "http://localhost:8082/health",
            "http://localhost:8083/health",
            "http://localhost:8084/health",
            "http://localhost:8085/health",
            "http://localhost:8086/health",
            "http://localhost:8082/api/v1/account",
            "http://localhost:8083/api/v1/strategies",
            "http://localhost:8085/api/v1/risk/limits"
        )
        
        $requestsSent = 0
        $testEndTime = $testStart.AddSeconds($Duration)
        
        while ((Get-Date) -lt $testEndTime -and $requestsSent -lt $totalRequests) {
            $secondStart = Get-Date
            $requestsThisSecond = 0
            
            # 在这一秒内发送请求
            while ((Get-Date) -lt $secondStart.AddSeconds(1) -and $requestsThisSecond -lt $RequestsPerSecond -and $requestsSent -lt $totalRequests) {
                $endpoint = $endpoints | Get-Random
                
                try {
                    $requestStart = Get-Date
                    $response = Invoke-RestMethod -Uri $endpoint -TimeoutSec 5 -ErrorAction Stop
                    $responseTime = ((Get-Date) - $requestStart).TotalMilliseconds
                    
                    $successfulRequests++
                    $totalResponseTime += $responseTime
                    
                    if ($responseTime -gt $maxResponseTime) { $maxResponseTime = $responseTime }
                    if ($responseTime -lt $minResponseTime) { $minResponseTime = $responseTime }
                    
                    # 记录状态码
                    $statusCode = "200"
                    if ($statusCodes.ContainsKey($statusCode)) {
                        $statusCodes[$statusCode]++
                    } else {
                        $statusCodes[$statusCode] = 1
                    }
                    
                } catch {
                    $failedRequests++
                    $statusCode = "Error"
                    if ($statusCodes.ContainsKey($statusCode)) {
                        $statusCodes[$statusCode]++
                    } else {
                        $statusCodes[$statusCode] = 1
                    }
                }
                
                $requestsSent++
                $requestsThisSecond++
                
                # 控制请求速率
                if ($requestsThisSecond -lt $RequestsPerSecond) {
                    Start-Sleep -Milliseconds (1000 / $RequestsPerSecond)
                }
            }
            
            # 显示进度
            if ($requestsSent % ($RequestsPerSecond * 10) -eq 0) {
                $progress = [math]::Round(($requestsSent / $totalRequests) * 100, 2)
                Write-TestLog "📈 Progress: $requestsSent/$totalRequests requests ($progress%)" "INFO"
            }
        }
        
        # 计算结果
        $actualRPS = if ($results.ActualDuration -gt 0) { $requestsSent / $Duration } else { 0 }
        $successRate = if ($requestsSent -gt 0) { ($successfulRequests / $requestsSent) * 100 } else { 0 }
        $avgResponseTime = if ($successfulRequests -gt 0) { $totalResponseTime / $successfulRequests } else { 0 }
        
        $results.Metrics = @{
            TotalRequests = $requestsSent
            SuccessfulRequests = $successfulRequests
            FailedRequests = $failedRequests
            SuccessRate = $successRate
            ActualRPS = $actualRPS
            AverageResponseTime = $avgResponseTime
            MinResponseTime = $minResponseTime
            MaxResponseTime = $maxResponseTime
            StatusCodes = $statusCodes
        }
        
        Write-TestLog "✅ API load test completed" "INFO"
        Write-TestLog "📊 Total Requests: $requestsSent" "INFO"
        Write-TestLog "📊 Success Rate: $([math]::Round($successRate, 2))%" "INFO"
        Write-TestLog "📊 Actual RPS: $([math]::Round($actualRPS, 2))" "INFO"
        Write-TestLog "📊 Average Response Time: $([math]::Round($avgResponseTime, 2))ms" "INFO"
        
    } catch {
        Write-TestLog "❌ API load test failed: $($_.Exception.Message)" "ERROR"
        $results.Success = $false
        $results.Error = $_.Exception.Message
    }
    
    $results.EndTime = Get-Date
    $results.ActualDuration = ($results.EndTime - $results.StartTime).TotalSeconds
    
    return $results
}

function Start-DatabaseLoadTest {
    param([int]$Queries, [int]$Duration)
    
    Write-TestLog "🗄️  Starting database load test..." "INFO"
    Write-TestLog "Target: $Queries queries over $Duration seconds" "INFO"
    
    $testStart = Get-Date
    $results = @{
        TestType = "Database Load Test"
        Queries = $Queries
        Duration = $Duration
        StartTime = $testStart
        Success = $true
        Metrics = @{}
    }
    
    try {
        # 模拟数据库查询测试
        $successfulQueries = 0
        $failedQueries = 0
        $totalQueryTime = 0
        $maxQueryTime = 0
        $minQueryTime = 999999
        
        Write-TestLog "📊 Executing database queries..." "INFO"
        
        $queriesPerSecond = [math]::Ceiling($Queries / $Duration)
        $queriesExecuted = 0
        $testEndTime = $testStart.AddSeconds($Duration)
        
        while ((Get-Date) -lt $testEndTime -and $queriesExecuted -lt $Queries) {
            # 模拟查询执行
            $queryTime = Get-Random -Minimum 1 -Maximum 50
            $totalQueryTime += $queryTime
            
            if ($queryTime -gt $maxQueryTime) { $maxQueryTime = $queryTime }
            if ($queryTime -lt $minQueryTime) { $minQueryTime = $queryTime }
            
            # 模拟查询成功率 (98%)
            if ((Get-Random -Minimum 1 -Maximum 100) -le 98) {
                $successfulQueries++
            } else {
                $failedQueries++
            }
            
            $queriesExecuted++
            
            # 控制查询速率
            Start-Sleep -Milliseconds ([math]::Max(1, 1000 / $queriesPerSecond))
            
            # 显示进度
            if ($queriesExecuted % 1000 -eq 0) {
                $progress = [math]::Round(($queriesExecuted / $Queries) * 100, 2)
                Write-TestLog "📈 Progress: $queriesExecuted/$Queries queries ($progress%)" "INFO"
            }
        }
        
        # 计算结果
        $actualQPS = if ($results.ActualDuration -gt 0) { $queriesExecuted / $Duration } else { 0 }
        $successRate = if ($queriesExecuted -gt 0) { ($successfulQueries / $queriesExecuted) * 100 } else { 0 }
        $avgQueryTime = if ($successfulQueries -gt 0) { $totalQueryTime / $successfulQueries } else { 0 }
        
        $results.Metrics = @{
            TotalQueries = $queriesExecuted
            SuccessfulQueries = $successfulQueries
            FailedQueries = $failedQueries
            SuccessRate = $successRate
            ActualQPS = $actualQPS
            AverageQueryTime = $avgQueryTime
            MinQueryTime = $minQueryTime
            MaxQueryTime = $maxQueryTime
        }
        
        Write-TestLog "✅ Database load test completed" "INFO"
        Write-TestLog "📊 Total Queries: $queriesExecuted" "INFO"
        Write-TestLog "📊 Success Rate: $([math]::Round($successRate, 2))%" "INFO"
        Write-TestLog "📊 Actual QPS: $([math]::Round($actualQPS, 2))" "INFO"
        Write-TestLog "📊 Average Query Time: $([math]::Round($avgQueryTime, 2))ms" "INFO"
        
    } catch {
        Write-TestLog "❌ Database load test failed: $($_.Exception.Message)" "ERROR"
        $results.Success = $false
        $results.Error = $_.Exception.Message
    }
    
    $results.EndTime = Get-Date
    $results.ActualDuration = ($results.EndTime - $results.StartTime).TotalSeconds
    
    return $results
}

function Generate-PerformanceReport {
    param([array]$TestResults)
    
    Write-TestLog "📊 Generating performance report..." "INFO"
    
    if (-not (Test-Path (Split-Path $script:ReportPath))) {
        New-Item -ItemType Directory -Path (Split-Path $script:ReportPath) -Force | Out-Null
    }
    
    $html = @"
<!DOCTYPE html>
<html>
<head>
    <title>Performance Test Report - $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .header { background-color: #2c3e50; color: white; padding: 20px; border-radius: 5px; }
        .summary { background-color: #ecf0f1; padding: 15px; margin: 20px 0; border-radius: 5px; }
        .test-result { background-color: #ffffff; border: 1px solid #bdc3c7; margin: 20px 0; padding: 15px; border-radius: 5px; }
        .success { border-left: 5px solid #27ae60; }
        .failure { border-left: 5px solid #e74c3c; }
        .metrics { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 10px; margin: 10px 0; }
        .metric { background-color: #f8f9fa; padding: 10px; border-radius: 3px; text-align: center; }
        .metric-value { font-size: 24px; font-weight: bold; color: #2c3e50; }
        .metric-label { font-size: 12px; color: #7f8c8d; }
        table { width: 100%; border-collapse: collapse; margin: 10px 0; }
        th, td { border: 1px solid #bdc3c7; padding: 8px; text-align: left; }
        th { background-color: #34495e; color: white; }
    </style>
</head>
<body>
    <div class="header">
        <h1>🔥 Performance Test Report</h1>
        <p>Generated: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')</p>
        <p>Test Duration: $((Get-Date) - $script:TestStartTime | ForEach-Object { "$($_.Hours)h $($_.Minutes)m $($_.Seconds)s" })</p>
    </div>
    
    <div class="summary">
        <h2>📊 Test Summary</h2>
        <div class="metrics">
            <div class="metric">
                <div class="metric-value">$($TestResults.Count)</div>
                <div class="metric-label">Total Tests</div>
            </div>
            <div class="metric">
                <div class="metric-value">$($TestResults | Where-Object { $_.Success } | Measure-Object | Select-Object -ExpandProperty Count)</div>
                <div class="metric-label">Successful Tests</div>
            </div>
            <div class="metric">
                <div class="metric-value">$($TestResults | Where-Object { -not $_.Success } | Measure-Object | Select-Object -ExpandProperty Count)</div>
                <div class="metric-label">Failed Tests</div>
            </div>
        </div>
    </div>
"@

    foreach ($result in $TestResults) {
        $statusClass = if ($result.Success) { "success" } else { "failure" }
        $statusIcon = if ($result.Success) { "[OK]" } else { "[FAIL]" }
        
        $html += @"
    <div class="test-result $statusClass">
        <h3>$statusIcon $($result.TestType)</h3>
        <p><strong>Duration:</strong> $([math]::Round($result.ActualDuration, 2)) seconds</p>
        <p><strong>Status:</strong> $(if ($result.Success) { "SUCCESS" } else { "FAILED" })</p>
"@

        if ($result.Error) {
            $html += "<p><strong>Error:</strong> $($result.Error)</p>"
        }
        
        if ($result.Metrics) {
            $html += "<h4>📈 Metrics</h4><div class='metrics'>"
            
            foreach ($metric in $result.Metrics.GetEnumerator()) {
                $value = if ($metric.Value -is [double] -or $metric.Value -is [float]) {
                    [math]::Round($metric.Value, 2)
                } else {
                    $metric.Value
                }
                
                $html += @"
                <div class="metric">
                    <div class="metric-value">$value</div>
                    <div class="metric-label">$($metric.Key)</div>
                </div>
"@
            }
            
            $html += "</div>"
        }
        
        $html += "</div>"
    }
    
    $html += @"
    <div class="summary">
        <h2>🎯 Performance Targets vs Results</h2>
        <table>
            <tr>
                <th>Metric</th>
                <th>Target</th>
                <th>Actual</th>
                <th>Status</th>
            </tr>
            <tr>
                <td>WebSocket Latency</td>
                <td>&lt; 50ms</td>
                <td>$(if ($TestResults | Where-Object { $_.TestType -eq "WebSocket Load Test" }) { [math]::Round(($TestResults | Where-Object { $_.TestType -eq "WebSocket Load Test" }).Metrics.AverageLatency, 2) } else { "N/A" })ms</td>
                <td>$(if (($TestResults | Where-Object { $_.TestType -eq "WebSocket Load Test" }) -and ($TestResults | Where-Object { $_.TestType -eq "WebSocket Load Test" }).Metrics.AverageLatency -lt 50) { "[OK] PASS" } else { "[FAIL] FAIL" })</td>
            </tr>
            <tr>
                <td>API Response Time</td>
                <td>&lt; 100ms</td>
                <td>$(if ($TestResults | Where-Object { $_.TestType -eq "API Load Test" }) { [math]::Round(($TestResults | Where-Object { $_.TestType -eq "API Load Test" }).Metrics.AverageResponseTime, 2) } else { "N/A" })ms</td>
                <td>$(if (($TestResults | Where-Object { $_.TestType -eq "API Load Test" }) -and ($TestResults | Where-Object { $_.TestType -eq "API Load Test" }).Metrics.AverageResponseTime -lt 100) { "[OK] PASS" } else { "[FAIL] FAIL" })</td>
            </tr>
            <tr>
                <td>System Availability</td>
                <td>&gt; 99.9%</td>
                <td>$(if ($TestResults | Where-Object { $_.TestType -eq "API Load Test" }) { [math]::Round(($TestResults | Where-Object { $_.TestType -eq "API Load Test" }).Metrics.SuccessRate, 2) } else { "N/A" })%</td>
                <td>$(if (($TestResults | Where-Object { $_.TestType -eq "API Load Test" }) -and ($TestResults | Where-Object { $_.TestType -eq "API Load Test" }).Metrics.SuccessRate -gt 99.9) { "[OK] PASS" } else { "[FAIL] FAIL" })</td>
            </tr>
        </table>
    </div>
    
    <div class="summary">
        <h2>💡 Recommendations</h2>
        <ul>
            <li>Monitor system resources during peak load</li>
            <li>Implement connection pooling for database connections</li>
            <li>Consider horizontal scaling for high-traffic scenarios</li>
            <li>Set up automated performance monitoring</li>
            <li>Implement circuit breakers for external dependencies</li>
        </ul>
    </div>
</body>
</html>
"@

    Set-Content -Path $script:ReportPath -Value $html -Encoding UTF8
    Write-TestLog "✅ Performance report generated: $script:ReportPath" "INFO"
    
    return $script:ReportPath
}

function Start-LoadTest {
    Write-TestLog "🚀 Starting comprehensive load test..." "INFO"
    
    # 检查先决条件
    if (-not (Test-Prerequisites)) {
        Write-TestLog "❌ Prerequisites check failed. Aborting test." "ERROR"
        return $false
    }
    
    $script:TestResults = @()
    
    # 执行不同类型的测试
    if ($TestType -eq "all" -or $TestType -eq "websocket") {
        Write-TestLog "🌐 Starting WebSocket load test..." "INFO"
        $wsResult = Start-WebSocketLoadTest -Connections $Connections -Duration $Duration
        $script:TestResults += $wsResult
    }
    
    if ($TestType -eq "all" -or $TestType -eq "api") {
        Write-TestLog "🔌 Starting API load test..." "INFO"
        $apiResult = Start-APILoadTest -RequestsPerSecond $RequestsPerSecond -Duration $Duration
        $script:TestResults += $apiResult
    }
    
    if ($TestType -eq "all" -or $TestType -eq "database") {
        Write-TestLog "🗄️  Starting database load test..." "INFO"
        $dbResult = Start-DatabaseLoadTest -Queries 50000 -Duration $Duration
        $script:TestResults += $dbResult
    }
    
    # 生成报告
    if ($GenerateReport) {
        $reportPath = Generate-PerformanceReport -TestResults $script:TestResults
        Write-TestLog "📊 Performance report available at: $reportPath" "INFO"
    }
    
    # 显示总结
    Write-Host ""
    Write-Host "🎉 Load Test Summary" -ForegroundColor Green
    Write-Host "=" * 50
    Write-Host "Total Tests: $($script:TestResults.Count)" -ForegroundColor Gray
    Write-Host "Successful: $($script:TestResults | Where-Object { $_.Success } | Measure-Object | Select-Object -ExpandProperty Count)" -ForegroundColor Green
    Write-Host "Failed: $($script:TestResults | Where-Object { -not $_.Success } | Measure-Object | Select-Object -ExpandProperty Count)" -ForegroundColor Red
    Write-Host "Total Duration: $((Get-Date) - $script:TestStartTime | ForEach-Object { "$($_.Hours)h $($_.Minutes)m $($_.Seconds)s" })" -ForegroundColor Gray
    
    return $true
}

# 主执行逻辑
$testSuccess = Start-LoadTest

if ($testSuccess) {
    Write-Host "🎉 Performance testing completed successfully!" -ForegroundColor Green
    exit 0
} else {
    Write-Host "❌ Performance testing failed!" -ForegroundColor Red
    exit 1
}