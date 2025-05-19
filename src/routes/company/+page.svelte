<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DevCorp Sim v1.0</title>
    <style>
        /* Base styles */
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: monospace;
            background-color: #09090b;
            color: #f4f4f5;
            line-height: 1.5;
            height: 100vh;
            overflow: hidden;
        }

        /* Layout */
        .app-container {
            display: flex;
            height: 100vh;
            width: 100%;
            overflow: hidden;
        }

        /* Sidebar */
        .sidebar {
            width: 280px;
            background-color: #18181b;
            border-right: 1px solid #27272a;
            display: flex;
            flex-direction: column;
            overflow: hidden;
            flex-shrink: 0;
        }

        .sidebar-header {
            padding: 1rem;
            border-bottom: 1px solid #27272a;
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }

        .sidebar-header-icon {
            color: #10b981;
            font-weight: bold;
        }

        .sidebar-header-title {
            font-size: 1.125rem;
            font-weight: bold;
        }

        .sidebar-content {
            flex: 1;
            overflow-y: auto;
            padding: 0.5rem;
        }

        .sidebar-menu {
            list-style: none;
        }

        .sidebar-menu-item {
            margin-bottom: 0.25rem;
        }

        .sidebar-menu-button {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.5rem;
            width: 100%;
            text-align: left;
            background: none;
            border: none;
            color: #f4f4f5;
            border-radius: 0.25rem;
            cursor: pointer;
            font-family: monospace;
        }

        .sidebar-menu-button:hover {
            background-color: #27272a;
        }

        .sidebar-menu-button.active {
            background-color: #27272a;
        }

        .sidebar-separator {
            height: 1px;
            background-color: #27272a;
            margin: 0.5rem 0;
        }

        .sidebar-group {
            margin-bottom: 1rem;
        }

        .sidebar-group-label {
            font-size: 0.75rem;
            color: #a1a1aa;
            padding: 0.5rem;
            font-weight: bold;
        }

        .game-controls {
            padding: 0.5rem;
        }

        .game-time {
            display: flex;
            align-items: center;
            justify-content: space-between;
            background-color: #09090b;
            border: 1px solid #27272a;
            border-radius: 0.25rem;
            padding: 0.5rem;
        }

        .game-time-info {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            font-size: 0.75rem;
            color: #a1a1aa;
        }

        .game-controls-buttons {
            display: flex;
            align-items: center;
            gap: 0.25rem;
        }

        .control-button {
            background-color: transparent;
            border: none;
            color: #f4f4f5;
            width: 1.5rem;
            height: 1.5rem;
            border-radius: 0.25rem;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            font-family: monospace;
        }

        .control-button:hover {
            background-color: #27272a;
        }

        .speed-button {
            background-color: transparent;
            border: none;
            color: #f4f4f5;
            padding: 0 0.5rem;
            height: 1.5rem;
            border-radius: 0.25rem;
            cursor: pointer;
            font-size: 0.75rem;
            font-family: monospace;
        }

        .speed-button:hover {
            background-color: #27272a;
        }

        .speed-button.active {
            background-color: #3f3f46;
        }

        .event-log {
            height: 200px;
            overflow-y: auto;
            background-color: #09090b;
            border: 1px solid #27272a;
            border-radius: 0.25rem;
            padding: 0.5rem;
            font-size: 0.75rem;
            margin-top: 0.5rem;
        }

        .event-item {
            display: flex;
            align-items: flex-start;
            gap: 0.5rem;
            margin-bottom: 0.5rem;
            padding-left: 0.5rem;
            border-left-width: 2px;
            border-left-style: solid;
        }

        .event-item.warning {
            border-left-color: #eab308;
        }

        .event-item.success {
            border-left-color: #10b981;
        }

        .event-item.info {
            border-left-color: #3b82f6;
        }

        .event-item.danger {
            border-left-color: #ef4444;
        }

        .event-item.special {
            border-left-color: #a855f7;
        }

        .event-time {
            color: #a1a1aa;
        }

        .sidebar-footer {
            padding: 1rem;
            border-top: 1px solid #27272a;
            display: flex;
            justify-content: space-between;
        }

        .footer-button {
            display: flex;
            align-items: center;
            gap: 0.25rem;
            padding: 0.25rem 0.5rem;
            background-color: #18181b;
            border: 1px solid #27272a;
            color: #f4f4f5;
            border-radius: 0.25rem;
            cursor: pointer;
            font-size: 0.75rem;
            font-family: monospace;
        }

        .footer-button:hover {
            background-color: #27272a;
        }

        /* Main content */
        .main-container {
            flex: 1;
            display: flex;
            flex-direction: column;
            overflow: hidden;
        }

        .header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0.5rem 1rem;
            background-color: #18181b;
            border-bottom: 1px solid #27272a;
        }

        .header-title {
            font-size: 1.125rem;
            font-weight: bold;
        }

        .header-stats {
            display: flex;
            align-items: center;
            gap: 1rem;
        }

        .stat-item {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            background-color: #09090b;
            border: 1px solid #27272a;
            border-radius: 0.25rem;
            padding: 0.25rem 0.75rem;
        }

        .stat-icon {
            color: #10b981;
        }

        .stat-icon.money {
            color: #10b981;
        }

        .stat-icon.reputation {
            color: #eab308;
        }

        .stat-icon.employees {
            color: #3b82f6;
        }

        .stat-value {
            font-size: 0.875rem;
            font-weight: 500;
        }

        .main-content {
            flex: 1;
            overflow: auto;
            padding: 1rem;
            background-color: #09090b;
            position: relative;
        }

        /* Tabs */
        .tabs {
            position: relative;
        }

        .tab-input {
            position: absolute;
            opacity: 0;
            z-index: -1;
        }

        .tab-controls {
            display: flex;
            margin-bottom: 1rem;
        }

        .tab-label {
            padding: 0.5rem 1rem;
            background-color: #18181b;
            color: #a1a1aa;
            cursor: pointer;
            border-radius: 0.25rem;
            margin-right: 0.5rem;
        }

        .tab-label:hover {
            background-color: #27272a;
        }

        .tab-input:checked + .tab-label {
            background-color: #3f3f46;
            color: #f4f4f5;
        }



        #tab-teams:checked ~ .tab-teams,
        #tab-projects:checked ~ .tab-projects,
        #tab-facilities:checked ~ .tab-facilities {
            display: block;
        }

        .tab-actions {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            margin-bottom: 1rem;
        }

        .action-button {
            display: flex;
            align-items: center;
            gap: 0.25rem;
            padding: 0.25rem 0.5rem;
            background-color: #18181b;
            border: 1px solid #27272a;
            color: #f4f4f5;
            border-radius: 0.25rem;
            cursor: pointer;
            font-size: 0.75rem;
            font-family: monospace;
        }

        .action-button:hover {
            background-color: #27272a;
        }

        /* Cards */
        .card {
            background-color: #18181b;
            border: 1px solid #27272a;
            border-radius: 0.5rem;
            overflow: hidden;
            margin-bottom: 1rem;
        }

        .card-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0.75rem 1rem;
            border-bottom: 1px solid #27272a;
        }

        .card-title {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            font-size: 1rem;
            font-weight: bold;
        }

        .card-title-icon {
            color: #10b981;
        }

        .card-title-icon.ba {
            color: #3b82f6;
        }

        .card-title-icon.qa {
            color: #a855f7;
        }

        .card-stats {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            font-size: 0.75rem;
            color: #a1a1aa;
        }

        .card-stat {
            display: flex;
            align-items: center;
            gap: 0.25rem;
        }

        .card-stat-icon.productivity {
            color: #10b981;
        }

        .card-stat-icon.morale {
            color: #ef4444;
        }

        .card-stat-icon.skill {
            color: #3b82f6;
        }

        /* Tables */
        .table-container {
            overflow-x: auto;
        }

        table {
            width: 100%;
            border-collapse: collapse;
            font-size: 0.875rem;
        }

        thead {
            background-color: #09090b;
            color: #d4d4d8;
        }

        th {
            padding: 0.5rem 1rem;
            text-align: left;
            font-weight: 500;
        }

        tbody tr {
            border-bottom: 1px solid #27272a;
            background-color: #18181b;
        }

        tbody tr:hover {
            background-color: #27272a;
        }

        td {
            padding: 0.5rem 1rem;
            color: #f4f4f5;
        }

        .employee-info {
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }

        .avatar {
            width: 1.5rem;
            height: 1.5rem;
            border-radius: 9999px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 0.75rem;
            font-weight: bold;
            color: #f4f4f5;
        }

        .avatar.dev1 {
            background-color: #10b981;
        }

        .avatar.dev2 {
            background-color: #3b82f6;
        }

        .avatar.dev3 {
            background-color: #a855f7;
        }

        .avatar.ba1 {
            background-color: #eab308;
        }

        .avatar.ba2 {
            background-color: #f97316;
        }

        .avatar.qa1 {
            background-color: #ef4444;
        }

        .avatar.qa2 {
            background-color: #10b981;
        }

        .task {
            display: flex;
            align-items: center;
            gap: 0.25rem;
        }

        .task-icon {
            color: #3b82f6;
        }

        .progress-container {
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }

        .progress-bar {
            width: 6rem;
            height: 0.5rem;
            background-color: #27272a;
            border-radius: 9999px;
            overflow: hidden;
        }

        .progress-value {
            height: 100%;
            background-color: #3b82f6;
            border-radius: 9999px;
        }

        .progress-text {
            font-size: 0.75rem;
        }

        .mood {
            display: flex;
            align-items: center;
            gap: 0.25rem;
        }

        .mood-icon.good {
            color: #10b981;
        }

        .mood-icon.average {
            color: #eab308;
        }

        .mood-icon.bad {
            color: #ef4444;
        }

        .mood-bar {
            width: 4rem;
            height: 0.5rem;
            background-color: #27272a;
            border-radius: 9999px;
            overflow: hidden;
        }

        .mood-value.good {
            height: 100%;
            background-color: #10b981;
            border-radius: 9999px;
        }

        .mood-value.average {
            height: 100%;
            background-color: #eab308;
            border-radius: 9999px;
        }

        .mood-value.bad {
            height: 100%;
            background-color: #ef4444;
            border-radius: 9999px;
        }

        .energy {
            display: flex;
            align-items: center;
            gap: 0.25rem;
        }

        .energy-icon.good {
            color: #10b981;
        }

        .energy-icon.average {
            color: #eab308;
        }

        .energy-icon.bad {
            color: #ef4444;
        }

        .energy-bar {
            width: 4rem;
            height: 0.5rem;
            background-color: #27272a;
            border-radius: 9999px;
            overflow: hidden;
        }

        .energy-value.good {
            height: 100%;
            background-color: #10b981;
            border-radius: 9999px;
        }

        .energy-value.average {
            height: 100%;
            background-color: #eab308;
            border-radius: 9999px;
        }

        .energy-value.bad {
            height: 100%;
            background-color: #ef4444;
            border-radius: 9999px;
        }

        .skills {
            display: flex;
            gap: 0.25rem;
        }

        .skill-tag {
            padding: 0 0.25rem;
            border-radius: 0.25rem;
            font-size: 0.75rem;
        }

        .skill-tag.js {
            background-color: rgba(59, 130, 246, 0.2);
            color: #93c5fd;
        }

        .skill-tag.react {
            background-color: rgba(168, 85, 247, 0.2);
            color: #d8b4fe;
        }

        .skill-tag.node {
            background-color: rgba(16, 185, 129, 0.2);
            color: #6ee7b7;
        }

        .skill-tag.css {
            background-color: rgba(236, 72, 153, 0.2);
            color: #f9a8d4;
        }

        .skill-tag.sql {
            background-color: rgba(16, 185, 129, 0.2);
            color: #6ee7b7;
        }

        .skill-tag.python {
            background-color: rgba(234, 179, 8, 0.2);
            color: #fde68a;
        }

        .skill-tag.agile {
            background-color: rgba(59, 130, 246, 0.2);
            color: #93c5fd;
        }

        .skill-tag.ux {
            background-color: rgba(249, 115, 22, 0.2);
            color: #fdba74;
        }

        .skill-tag.jira {
            background-color: rgba(16, 185, 129, 0.2);
            color: #6ee7b7;
        }

        .skill-tag.scrum {
            background-color: rgba(168, 85, 247, 0.2);
            color: #d8b4fe;
        }

        .skill-tag.testing {
            background-color: rgba(239, 68, 68, 0.2);
            color: #fca5a5;
        }

        .skill-tag.selenium {
            background-color: rgba(59, 130, 246, 0.2);
            color: #93c5fd;
        }

        .skill-tag.cicd {
            background-color: rgba(234, 179, 8, 0.2);
            color: #fde68a;
        }

        .status-badge {
            display: inline-block;
            padding: 0.125rem 0.5rem;
            border-radius: 9999px;
            font-size: 0.75rem;
        }

        .status-badge.focused {
            background-color: rgba(16, 185, 129, 0.2);
            color: #6ee7b7;
        }

        .status-badge.tired {
            background-color: rgba(239, 68, 68, 0.2);
            color: #fca5a5;
        }

        .status-badge.inspired {
            background-color: rgba(59, 130, 246, 0.2);
            color: #93c5fd;
        }

        .status-badge.burnout {
            background-color: rgba(239, 68, 68, 0.2);
            color: #fca5a5;
        }

        .status-badge.productive {
            background-color: rgba(16, 185, 129, 0.2);
            color: #6ee7b7;
        }

        .status-badge.concerned {
            background-color: rgba(234, 179, 8, 0.2);
            color: #fde68a;
        }

        /* Footer */
        .footer {
            padding: 0.5rem 1rem;
            background-color: #18181b;
            border-top: 1px solid #27272a;
            font-size: 0.75rem;
            color: #a1a1aa;
            display: flex;
            align-items: center;
            justify-content: space-between;
        }

        .footer-info {
            display: flex;
            align-items: center;
            gap: 1rem;
        }

        .footer-item {
            display: flex;
            align-items: center;
            gap: 0.25rem;
        }

        /* Collapsible sections */


        .team-toggle:checked + .card-header + .team-content {
            display: block;
        }

        .chevron {
            transition: transform 0.2s;
        }

        .team-toggle:checked + .card-header .chevron {
            transform: rotate(90deg);
        }

        /* Document Tabs System */
        .document-tabs {
            display: flex;
            background-color: #18181b;
            border-bottom: 1px solid #27272a;
            overflow-x: auto;
            white-space: nowrap;
        }

        .document-tab-input {
            position: absolute;
            opacity: 0;
            z-index: -1;
        }

        .document-tab-label {
            display: inline-flex;
            align-items: center;
            padding: 0.5rem 1rem;
            background-color: #27272a;
            color: #a1a1aa;
            cursor: pointer;
            border-right: 1px solid #3f3f46;
            user-select: none;
            position: relative;
        }

        .document-tab-label:hover {
            background-color: #3f3f46;
        }

        .document-tab-input:checked + .document-tab-label {
            background-color: #3f3f46;
            color: #f4f4f5;
            border-bottom: 2px solid #10b981;
        }

        .document-tab-close {
            margin-left: 0.5rem;
            width: 1rem;
            height: 1rem;
            border-radius: 50%;
            display: inline-flex;
            align-items: center;
            justify-content: center;
            font-size: 0.75rem;
            background-color: #3f3f46;
            color: #a1a1aa;
        }

        .document-tab-close:hover {
            background-color: #ef4444;
            color: #f4f4f5;
        }

        .document-content {
            /*display: none;*/
            padding: 1rem;
            background-color: #18181b;
            border-radius: 0.25rem;
            margin-top: 1rem;
        }

        .document-tab-input:checked ~ .document-content {
            display: block;
        }

        .document-tab-input:not(:checked) ~ .document-content {
            /*display: none;*/
        }

        /* Employee Detail View */
        .employee-detail {
            display: flex;
            flex-direction: column;
            gap: 1rem;
        }

        .employee-detail-header {
            display: flex;
            align-items: center;
            gap: 1rem;
            padding-bottom: 1rem;
            border-bottom: 1px solid #27272a;
        }

        .employee-detail-avatar {
            width: 4rem;
            height: 4rem;
            border-radius: 9999px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1.5rem;
            font-weight: bold;
        }

        .employee-detail-info {
            flex: 1;
        }

        .employee-detail-name {
            font-size: 1.5rem;
            font-weight: bold;
            margin-bottom: 0.25rem;
        }

        .employee-detail-role {
            color: #a1a1aa;
            margin-bottom: 0.5rem;
        }

        .employee-detail-stats {
            display: flex;
            gap: 1rem;
            flex-wrap: wrap;
        }

        .employee-detail-stat {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            background-color: #27272a;
            padding: 0.25rem 0.75rem;
            border-radius: 0.25rem;
        }

        .employee-detail-section {
            margin-top: 1rem;
        }

        .employee-detail-section-title {
            font-size: 1rem;
            font-weight: bold;
            margin-bottom: 0.5rem;
            color: #d4d4d8;
        }

        .employee-detail-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
            gap: 1rem;
        }

        .employee-detail-card {
            background-color: #27272a;
            border-radius: 0.25rem;
            padding: 1rem;
        }

        .employee-detail-card-title {
            font-size: 0.875rem;
            font-weight: bold;
            margin-bottom: 0.5rem;
            color: #d4d4d8;
        }

        .employee-detail-progress {
            margin-top: 0.5rem;
        }

        .employee-detail-progress-label {
            display: flex;
            justify-content: space-between;
            margin-bottom: 0.25rem;
            font-size: 0.75rem;
        }

        .employee-detail-progress-bar {
            height: 0.5rem;
            background-color: #3f3f46;
            border-radius: 9999px;
            overflow: hidden;
        }

        .employee-detail-progress-value {
            height: 100%;
            border-radius: 9999px;
        }

        .employee-detail-progress-value.good {
            background-color: #10b981;
        }

        .employee-detail-progress-value.average {
            background-color: #eab308;
        }

        .employee-detail-progress-value.bad {
            background-color: #ef4444;
        }

        .employee-detail-history {
            margin-top: 1rem;
        }

        .history-item {
            display: flex;
            gap: 0.5rem;
            padding: 0.5rem 0;
            border-bottom: 1px solid #3f3f46;
        }

        .history-item:last-child {
            border-bottom: none;
        }

        .history-date {
            color: #a1a1aa;
            font-size: 0.75rem;
            min-width: 5rem;
        }

        .history-description {
            flex: 1;
        }

        /* Employee row clickable */
        .employee-row {
            cursor: pointer;
        }

        .employee-row:hover {
            background-color: #3f3f46;
        }

        /* Utility classes */
        .flex {
            display: flex;
        }

        .items-center {
            align-items: center;
        }

        .gap-1 {
            gap: 0.25rem;
        }

        .gap-2 {
            gap: 0.5rem;
        }

        .justify-between {
            justify-content: space-between;
        }

        .mb-4 {
            margin-bottom: 1rem;
        }

        /* Dashboard view */
        .dashboard-view {
            display: block;
        }

        .document-tab-input:checked ~ .dashboard-view {
            display: none;
        }

        /* New tab button */
        .new-tab-button {
            display: inline-flex;
            align-items: center;
            padding: 0.5rem 1rem;
            background-color: #18181b;
            color: #a1a1aa;
            cursor: pointer;
            border-right: 1px solid #3f3f46;
        }

        .new-tab-button:hover {
            background-color: #27272a;
        }
    </style>
</head>
<body>
<div class="app-container">
    <!-- Sidebar -->
    <div class="sidebar">
        <div class="sidebar-header">
            <span class="sidebar-header-icon">⌨️</span>
            <span class="sidebar-header-title">DevCorp Sim v1.0</span>
        </div>

        <div class="sidebar-content">
            <ul class="sidebar-menu">
                <li class="sidebar-menu-item">
                    <button class="sidebar-menu-button active">
                        <span>🏠</span>
                        <span>Dashboard</span>
                    </button>
                </li>
                <li class="sidebar-menu-item">
                    <button class="sidebar-menu-button" >
                        <span>🏢</span>
                        <span>Company Overview</span>
                    </button>
                </li>
                <li class="sidebar-menu-item">
                    <button class="sidebar-menu-button" >
                        <span>📁</span>
                        <span>Project Overview</span>
                    </button>
                </li>
                <li class="sidebar-menu-item">
                    <button class="sidebar-menu-button">
                        <span>👥</span>
                        <span>Personnel</span>
                    </button>
                </li>
                <li class="sidebar-menu-item">
                    <button class="sidebar-menu-button">
                        <span>📁</span>
                        <span>Projects</span>
                    </button>
                </li>
                <li class="sidebar-menu-item">
                    <button class="sidebar-menu-button">
                        <span>🧠</span>
                        <span>Research</span>
                    </button>
                </li>
                <li class="sidebar-menu-item">
                    <button class="sidebar-menu-button">
                        <span>💰</span>
                        <span>Finances</span>
                    </button>
                </li>
                <li class="sidebar-menu-item">
                    <button class="sidebar-menu-button">
                        <span>📊</span>
                        <span>Analytics</span>
                    </button>
                </li>
            </ul>

            <div class="sidebar-separator"></div>

            <div class="sidebar-group">
                <div class="sidebar-group-label">Game Controls</div>
                <div class="game-controls">
                    <div class="game-time">
                        <div class="game-time-info">
                            <span>🕒</span>
                            <span>Day 42 - Q2 2025</span>
                        </div>
                        <div class="game-controls-buttons">
                            <button class="control-button">▶️</button>
                            <button class="speed-button active">1x</button>
                            <button class="speed-button">2x</button>
                            <button class="speed-button">3x</button>
                        </div>
                    </div>
                </div>
            </div>

            <div class="sidebar-separator"></div>

            <div class="sidebar-group">
                <div class="sidebar-group-label">Event Log</div>
                <div class="event-log">
                    <div class="event-item warning">
                        <span>⚠️</span>
                        <div>
                            <span class="event-time">09:45</span> - Sarah is experiencing burnout. Productivity -15%
                        </div>
                    </div>
                    <div class="event-item success">
                        <span>⚡</span>
                        <div>
                            <span class="event-time">09:30</span> - Project Alpha milestone completed! Team morale +10%
                        </div>
                    </div>
                    <div class="event-item info">
                        <span>💡</span>
                        <div>
                            <span class="event-time">09:15</span> - Mike had a breakthrough idea. Innovation +5
                        </div>
                    </div>
                    <div class="event-item danger">
                        <span>🔥</span>
                        <div>
                            <span class="event-time">09:00</span> - Critical bug discovered in Project Beta. Quality -10%
                        </div>
                    </div>
                    <div class="event-item special">
                        <span>☕</span>
                        <div>
                            <span class="event-time">08:45</span> - Coffee machine upgraded. Team happiness +5%
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div class="sidebar-footer">
            <button class="footer-button">
                <span>⚙️</span>
                <span>Settings</span>
            </button>
            <button class="footer-button">
                <span>❓</span>
                <span>Help</span>
            </button>
        </div>
    </div>

    <!-- Main Content -->
    <div class="main-container">
        <header class="header">
            <div class="header-title">Company Dashboard</div>
            <div class="header-stats">
                <div class="stat-item">
                    <span class="stat-icon money">💰</span>
                    <span class="stat-value">$1,245,678</span>
                </div>
                <div class="stat-item">
                    <span class="stat-icon reputation">⭐</span>
                    <span class="stat-value">Company Rep: 78%</span>
                </div>
                <div class="stat-item">
                    <span class="stat-icon employees">👥</span>
                    <span class="stat-value">Employees: 24</span>
                </div>
            </div>
        </header>

        <!-- Document Tabs Bar -->
        <div class="document-tabs">
            <label for="dashboard-tab" class="document-tab-label">
                <span>🏠 Dashboard</span>
            </label>

            <input type="radio" name="document-tabs" id="employee-tab-1" class="document-tab-input">
            <label for="employee-tab-1" class="document-tab-label">
                <span>👤 John Doe</span>
                <label for="dashboard-tab" class="document-tab-close">×</label>
            </label>

            <input type="radio" name="document-tabs" id="employee-tab-2" class="document-tab-input">
            <label for="employee-tab-2" class="document-tab-label">
                <span>👤 Jane Smith</span>
                <label for="dashboard-tab" class="document-tab-close">×</label>
            </label>

            <input type="radio" name="document-tabs" id="employee-tab-3" class="document-tab-input">
            <label for="employee-tab-3" class="document-tab-label">
                <span>👤 Mike Johnson</span>
                <label for="dashboard-tab" class="document-tab-close">×</label>
            </label>

            <input type="radio" name="document-tabs" id="project-overview-tab" class="document-tab-input">
            <label for="project-overview-tab" class="document-tab-label">
                <span>📁 Project Overview</span>
                <label for="dashboard-tab" class="document-tab-close">×</label>
            </label>

            <input type="radio" name="document-tabs" id="company-overview-tab" class="document-tab-input">
            <label for="company-overview-tab" class="document-tab-label">
                <span>🏢 Company Overview</span>
                <label for="dashboard-tab" class="document-tab-close">×</label>
            </label>

            <input type="radio" name="document-tabs" id="dashboard-tab" class="document-tab-input" checked>
        </div>

        <main class="main-content">
            <!-- Dashboard View -->


            <!-- Project Overview Tab -->
            <div class="document-content">
                <div class="employee-detail">
                    <div class="employee-detail-header">
                        <div style="font-size: 2rem;">📁</div>
                        <div class="employee-detail-info">
                            <div class="employee-detail-name">Project Overview</div>
                            <div class="employee-detail-role">All active and upcoming projects</div>
                            <div class="employee-detail-stats">
                                <div class="employee-detail-stat">
                                    <span>📊</span>
                                    <span>Active Projects: 5</span>
                                </div>
                                <div class="employee-detail-stat">
                                    <span>⏱️</span>
                                    <span>On Schedule: 4</span>
                                </div>
                                <div class="employee-detail-stat">
                                    <span>💰</span>
                                    <span>Budget: $2.4M</span>
                                </div>
                                <div class="employee-detail-stat">
                                    <span>👥</span>
                                    <span>Team Members: 18</span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="tab-actions" style="margin-top: 1rem;">
                        <button class="action-button">
                            <span>➕</span>
                            <span>New Project</span>
                        </button>
                        <button class="action-button">
                            <span>📊</span>
                            <span>Project Analytics</span>
                        </button>
                        <button class="action-button">
                            <span>⚙️</span>
                            <span>Settings</span>
                        </button>
                    </div>

                    <!-- Active Projects -->
                    <div class="employee-detail-section">
                        <div class="employee-detail-section-title">Active Projects</div>

                        <!-- Project 1 -->
                        <div class="card" style="margin-bottom: 1rem;">
                            <div class="card-header">
                                <div class="card-title">
                                    <span class="card-title-icon">🚀</span>
                                    <span>Customer Portal</span>
                                </div>
                                <div class="card-stats">
                                    <div class="card-stat">
                                        <span class="card-stat-icon productivity">📈</span>
                                        <span>Progress: 68%</span>
                                    </div>
                                    <div class="card-stat">
                                        <span class="card-stat-icon skill">⏱️</span>
                                        <span>Deadline: Day 65</span>
                                    </div>
                                    <div class="card-stat">
                                        <span class="card-stat-icon morale">💰</span>
                                        <span>Budget: $450K</span>
                                    </div>
                                </div>
                            </div>
                            <div style="padding: 1rem;">
                                <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 1rem;">
                                    <div class="employee-detail-card">
                                        <div class="employee-detail-card-title">Project Details</div>
                                        <p style="margin-bottom: 0.5rem; color: #d4d4d8;">A customer-facing portal for account management and service requests.</p>
                                        <div style="display: flex; gap: 0.5rem; margin-top: 0.5rem;">
                                            <span class="skill-tag js">High Priority</span>
                                            <span class="skill-tag react">External</span>
                                        </div>
                                    </div>

                                    <div class="employee-detail-card">
                                        <div class="employee-detail-card-title">Team Allocation</div>
                                        <div style="display: flex; gap: 0.5rem; margin-bottom: 0.5rem;">
                                            <div class="avatar dev1">JD</div>
                                            <div class="avatar dev2">JS</div>
                                            <div class="avatar dev3">MJ</div>
                                            <div class="avatar ba1">SL</div>
                                            <div class="avatar qa1">TK</div>
                                            <div style="width: 1.5rem; height: 1.5rem; border-radius: 9999px; background-color: #27272a; display: flex; align-items: center; justify-content: center; font-size: 0.75rem;">+2</div>
                                        </div>
                                        <div class="employee-detail-progress">
                                            <div class="employee-detail-progress-label">
                                                <span>Resource Allocation</span>
                                                <span>75%</span>
                                            </div>
                                            <div class="employee-detail-progress-bar">
                                                <div class="employee-detail-progress-value good" style="width: 75%;"></div>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="employee-detail-card">
                                        <div class="employee-detail-card-title">Milestones</div>
                                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.25rem;">
                                            <span>Backend API</span>
                                            <span class="status-badge focused">Completed</span>
                                        </div>
                                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.25rem;">
                                            <span>Frontend UI</span>
                                            <span class="status-badge inspired">In Progress</span>
                                        </div>
                                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.25rem;">
                                            <span>Database Optimization</span>
                                            <span class="status-badge inspired">In Progress</span>
                                        </div>
                                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.25rem;">
                                            <span>Testing & QA</span>
                                            <span class="status-badge concerned">Not Started</span>
                                        </div>
                                    </div>
                                </div>

                                <div style="margin-top: 1rem;">
                                    <div class="employee-detail-progress">
                                        <div class="employee-detail-progress-label">
                                            <span>Overall Progress</span>
                                            <span>68%</span>
                                        </div>
                                        <div class="employee-detail-progress-bar">
                                            <div class="employee-detail-progress-value average" style="width: 68%;"></div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <!-- Project 2 -->
                        <div class="card" style="margin-bottom: 1rem;">
                            <div class="card-header">
                                <div class="card-title">
                                    <span class="card-title-icon">📱</span>
                                    <span>Mobile App Redesign</span>
                                </div>
                                <div class="card-stats">
                                    <div class="card-stat">
                                        <span class="card-stat-icon productivity">📈</span>
                                        <span>Progress: 92%</span>
                                    </div>
                                    <div class="card-stat">
                                        <span class="card-stat-icon skill">⏱️</span>
                                        <span>Deadline: Day 45</span>
                                    </div>
                                    <div class="card-stat">
                                        <span class="card-stat-icon morale">💰</span>
                                        <span>Budget: $320K</span>
                                    </div>
                                </div>
                            </div>
                            <div style="padding: 1rem;">
                                <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 1rem;">
                                    <div class="employee-detail-card">
                                        <div class="employee-detail-card-title">Project Details</div>
                                        <p style="margin-bottom: 0.5rem; color: #d4d4d8;">Complete redesign of our mobile application with improved UX and new features.</p>
                                        <div style="display: flex; gap: 0.5rem; margin-top: 0.5rem;">
                                            <span class="skill-tag js">High Priority</span>
                                            <span class="skill-tag react">External</span>
                                        </div>
                                    </div>

                                    <div class="employee-detail-card">
                                        <div class="employee-detail-card-title">Team Allocation</div>
                                        <div style="display: flex; gap: 0.5rem; margin-bottom: 0.5rem;">
                                            <div class="avatar dev2">JS</div>
                                            <div class="avatar ba2">RL</div>
                                            <div class="avatar qa2">AW</div>
                                            <div style="width: 1.5rem; height: 1.5rem; border-radius: 9999px; background-color: #27272a; display: flex; align-items: center; justify-content: center; font-size: 0.75rem;">+3</div>
                                        </div>
                                        <div class="employee-detail-progress">
                                            <div class="employee-detail-progress-label">
                                                <span>Resource Allocation</span>
                                                <span>90%</span>
                                            </div>
                                            <div class="employee-detail-progress-bar">
                                                <div class="employee-detail-progress-value good" style="width: 90%;"></div>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="employee-detail-card">
                                        <div class="employee-detail-card-title">Milestones</div>
                                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.25rem;">
                                            <span>UX Research</span>
                                            <span class="status-badge focused">Completed</span>
                                        </div>
                                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.25rem;">
                                            <span>UI Design</span>
                                            <span class="status-badge focused">Completed</span>
                                        </div>
                                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.25rem;">
                                            <span>Frontend Implementation</span>
                                            <span class="status-badge focused">Completed</span>
                                        </div>
                                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.25rem;">
                                            <span>Testing & QA</span>
                                            <span class="status-badge inspired">In Progress</span>
                                        </div>
                                    </div>
                                </div>

                                <div style="margin-top: 1rem;">
                                    <div class="employee-detail-progress">
                                        <div class="employee-detail-progress-label">
                                            <span>Overall Progress</span>
                                            <span>92%</span>
                                        </div>
                                        <div class="employee-detail-progress-bar">
                                            <div class="employee-detail-progress-value good" style="width: 92%;"></div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <!-- Project 3 -->
                        <div class="card" style="margin-bottom: 1rem;">
                            <div class="card-header">
                                <div class="card-title">
                                    <span class="card-title-icon">🔒</span>
                                    <span>Security Audit</span>
                                </div>
                                <div class="card-stats">
                                    <div class="card-stat">
                                        <span class="card-stat-icon productivity">📈</span>
                                        <span>Progress: 45%</span>
                                    </div>
                                    <div class="card-stat">
                                        <span class="card-stat-icon skill">⏱️</span>
                                        <span>Deadline: Day 70</span>
                                    </div>
                                    <div class="card-stat">
                                        <span class="card-stat-icon morale">💰</span>
                                        <span>Budget: $180K</span>
                                    </div>
                                </div>
                            </div>
                            <div style="padding: 1rem;">
                                <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 1rem;">
                                    <div class="employee-detail-card">
                                        <div class="employee-detail-card-title">Project Details</div>
                                        <p style="margin-bottom: 0.5rem; color: #d4d4d8;">Comprehensive security audit of all systems and implementation of security improvements.</p>
                                        <div style="display: flex; gap: 0.5rem; margin-top: 0.5rem;">
                                            <span class="skill-tag js">Medium Priority</span>
                                            <span class="skill-tag python">Internal</span>
                                        </div>
                                    </div>

                                    <div class="employee-detail-card">
                                        <div class="employee-detail-card-title">Team Allocation</div>
                                        <div style="display: flex; gap: 0.5rem; margin-bottom: 0.5rem;">
                                            <div class="avatar dev1">JD</div>
                                            <div class="avatar dev3">MJ</div>
                                            <div style="width: 1.5rem; height: 1.5rem; border-radius: 9999px; background-color: #27272a; display: flex; align-items: center; justify-content: center; font-size: 0.75rem;">+2</div>
                                        </div>
                                        <div class="employee-detail-progress">
                                            <div class="employee-detail-progress-label">
                                                <span>Resource Allocation</span>
                                                <span>50%</span>
                                            </div>
                                            <div class="employee-detail-progress-bar">
                                                <div class="employee-detail-progress-value average" style="width: 50%;"></div>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="employee-detail-card">
                                        <div class="employee-detail-card-title">Milestones</div>
                                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.25rem;">
                                            <span>Initial Assessment</span>
                                            <span class="status-badge focused">Completed</span>
                                        </div>
                                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.25rem;">
                                            <span>Vulnerability Scanning</span>
                                            <span class="status-badge inspired">In Progress</span>
                                        </div>
                                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.25rem;">
                                            <span>Remediation</span>
                                            <span class="status-badge concerned">Not Started</span>
                                        </div>
                                        <div style="display: flex; justify-content: space-between; margin-bottom: 0.25rem;">
                                            <span>Final Report</span>
                                            <span class="status-badge concerned">Not Started</span>
                                        </div>
                                    </div>
                                </div>

                                <div style="margin-top: 1rem;">
                                    <div class="employee-detail-progress">
                                        <div class="employee-detail-progress-label">
                                            <span>Overall Progress</span>
                                            <span>45%</span>
                                        </div>
                                        <div class="employee-detail-progress-bar">
                                            <div class="employee-detail-progress-value average" style="width: 45%;"></div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Upcoming Projects -->
                    <div class="employee-detail-section">
                        <div class="employee-detail-section-title">Upcoming Projects</div>

                        <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 1rem;">
                            <!-- Upcoming Project 1 -->
                            <div class="employee-detail-card">
                                <div class="employee-detail-card-title">AI Recommendation Engine</div>
                                <p style="margin-bottom: 0.5rem; color: #d4d4d8;">Implementation of AI-based recommendation system for product suggestions.</p>
                                <div style="display: flex; justify-content: space-between; margin-top: 0.5rem;">
                                    <span>Start Date: Day 75</span>
                                    <span>Budget: $550K</span>
                                </div>
                                <div style="display: flex; gap: 0.5rem; margin-top: 0.5rem;">
                                    <span class="skill-tag js">High Priority</span>
                                    <span class="skill-tag python">External</span>
                                </div>
                            </div>

                            <!-- Upcoming Project 2 -->
                            <div class="employee-detail-card">
                                <div class="employee-detail-card-title">Data Warehouse Migration</div>
                                <p style="margin-bottom: 0.5rem; color: #d4d4d8;">Migration of existing data warehouse to cloud-based solution.</p>
                                <div style="display: flex; justify-content: space-between; margin-top: 0.5rem;">
                                    <span>Start Date: Day 80</span>
                                    <span>Budget: $320K</span>
                                </div>
                                <div style="display: flex; gap: 0.5rem; margin-top: 0.5rem;">
                                    <span class="skill-tag sql">Medium Priority</span>
                                    <span class="skill-tag python">Internal</span>
                                </div>
                            </div>

                            <!-- Upcoming Project 3 -->
                            <div class="employee-detail-card">
                                <div class="employee-detail-card-title">DevOps Pipeline Upgrade</div>
                                <p style="margin-bottom: 0.5rem; color: #d4d4d8;">Modernization of CI/CD pipeline and deployment processes.</p>
                                <div style="display: flex; justify-content: space-between; margin-top: 0.5rem;">
                                    <span>Start Date: Day 90</span>
                                    <span>Budget: $180K</span>
                                </div>
                                <div style="display: flex; gap: 0.5rem; margin-top: 0.5rem;">
                                    <span class="skill-tag cicd">Low Priority</span>
                                    <span class="skill-tag python">Internal</span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Project Risks -->
                    <div class="employee-detail-section">
                        <div class="employee-detail-section-title">Project Risks & Issues</div>

                        <div class="table-container">
                            <table>
                                <thead>
                                <tr>
                                    <th>Project</th>
                                    <th>Risk/Issue</th>
                                    <th>Impact</th>
                                    <th>Probability</th>
                                    <th>Mitigation</th>
                                    <th>Status</th>
                                </tr>
                                </thead>
                                <tbody>
                                <tr>
                                    <td>Customer Portal</td>
                                    <td>API performance bottleneck</td>
                                    <td>High</td>
                                    <td>Medium</td>
                                    <td>Implement caching and optimize database queries</td>
                                    <td><span class="status-badge inspired">In Progress</span></td>
                                </tr>
                                <tr>
                                    <td>Mobile App Redesign</td>
                                    <td>UX inconsistencies</td>
                                    <td>Medium</td>
                                    <td>Low</td>
                                    <td>Additional UX review and testing</td>
                                    <td><span class="status-badge focused">Resolved</span></td>
                                </tr>
                                <tr>
                                    <td>Security Audit</td>
                                    <td>Resource constraints</td>
                                    <td>High</td>
                                    <td>High</td>
                                    <td>Request additional resources or extend timeline</td>
                                    <td><span class="status-badge concerned">Open</span></td>
                                </tr>
                                <tr>
                                    <td>AI Recommendation Engine</td>
                                    <td>Data quality issues</td>
                                    <td>High</td>
                                    <td>Medium</td>
                                    <td>Data cleansing initiative before project start</td>
                                    <td><span class="status-badge concerned">Open</span></td>
                                </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            </div>

            <!-- Company Overview Tab -->
            <div class="document-content">
                <div class="employee-detail">
                    <div class="employee-detail-header">
                        <div style="font-size: 2rem;">🏢</div>
                        <div class="employee-detail-info">
                            <div class="employee-detail-name">DevCorp Inc.</div>
                            <div class="employee-detail-role">Software Development & Consulting</div>
                            <div class="employee-detail-stats">
                                <div class="employee-detail-stat">
                                    <span>💰</span>
                                    <span>Revenue: $12.4M</span>
                                </div>
                                <div class="employee-detail-stat">
                                    <span>📈</span>
                                    <span>Growth: +18%</span>
                                </div>
                                <div class="employee-detail-stat">
                                    <span>👥</span>
                                    <span>Employees: 24</span>
                                </div>
                                <div class="employee-detail-stat">
                                    <span>⭐</span>
                                    <span>Reputation: 78%</span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Company Performance -->
                    <div class="employee-detail-section">
                        <div class="employee-detail-section-title">Company Performance</div>

                        <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(250px, 1fr)); gap: 1rem;">
                            <!-- Financial Health -->
                            <div class="employee-detail-card">
                                <div class="employee-detail-card-title">Financial Health</div>
                                <div class="employee-detail-progress">
                                    <div class="employee-detail-progress-label">
                                        <span>Revenue</span>
                                        <span>$12.4M</span>
                                    </div>
                                    <div class="employee-detail-progress-bar">
                                        <div class="employee-detail-progress-value good" style="width: 85%;"></div>
                                    </div>
                                </div>
                                <div class="employee-detail-progress">
                                    <div class="employee-detail-progress-label">
                                        <span>Expenses</span>
                                        <span>$8.2M</span>
                                    </div>
                                    <div class="employee-detail-progress-bar">
                                        <div class="employee-detail-progress-value average" style="width: 65%;"></div>
                                    </div>
                                </div>
                                <div class="employee-detail-progress">
                                    <div class="employee-detail-progress-label">
                                        <span>Profit Margin</span>
                                        <span>34%</span>
                                    </div>
                                    <div class="employee-detail-progress-bar">
                                        <div class="employee-detail-progress-value good" style="width: 75%;"></div>
                                    </div>
                                </div>
                                <div class="employee-detail-progress">
                                    <div class="employee-detail-progress-label">
                                        <span>Cash Reserves</span>
                                        <span>$3.2M</span>
                                    </div>
                                    <div class="employee-detail-progress-bar">
                                        <div class="employee-detail-progress-value good" style="width: 80%;"></div>
                                    </div>
                                </div>
                            </div>

                            <!-- Customer Metrics -->
                            <div class="employee-detail-card">
                                <div class="employee-detail-card-title">Customer Metrics</div>
                                <div class="employee-detail-progress">
                                    <div class="employee-detail-progress-label">
                                        <span>Active Clients</span>
                                        <span>18</span>
                                    </div>
                                    <div class="employee-detail-progress-bar">
                                        <div class="employee-detail-progress-value good" style="width: 90%;"></div>
                                    </div>
                                </div>
                                <div class="employee-detail-progress">
                                    <div class="employee-detail-progress-label">
                                        <span>Client Satisfaction</span>
                                        <span>85%</span>
                                    </div>
                                    <div class="employee-detail-progress-bar">
                                        <div class="employee-detail-progress-value good" style="width: 85%;"></div>
                                    </div>
                                </div>
                                <div class="employee-detail-progress">
                                    <div class="employee-detail-progress-label">
                                        <span>Retention Rate</span>
                                        <span>92%</span>
                                    </div>
                                    <div class="employee-detail-progress-bar">
                                        <div class="employee-detail-progress-value good" style="width: 92%;"></div>
                                    </div>
                                </div>
                                <div class="employee-detail-progress">
                                    <div class="employee-detail-progress-label">
                                        <span>New Leads</span>
                                        <span>12</span>
                                    </div>
                                    <div class="employee-detail-progress-bar">
                                        <div class="employee-detail-progress-value average" style="width: 60%;"></div>
                                    </div>
                                </div>
                            </div>

                            <!-- Team Performance -->
                            <div class="employee-detail-card">
                                <div class="employee-detail-card-title">Team Performance</div>
                                <div class="employee-detail-progress">
                                    <div class="employee-detail-progress-label">
                                        <span>Development</span>
                                        <span>87%</span>
                                    </div>
                                    <div class="employee-detail-progress-bar">
                                        <div class="employee-detail-progress-value good" style="width: 87%;"></div>
                                    </div>
                                </div>
                                <div class="employee-detail-progress">
                                    <div class="employee-detail-progress-label">
                                        <span>Business Analysis</span>
                                        <span>92%</span>
                                    </div>
                                    <div class="employee-detail-progress-bar">
                                        <div class="employee-detail-progress-value good" style="width: 92%;"></div>
                                    </div>
                                </div>
                                <div class="employee-detail-progress">
                                    <div class="employee-detail-progress-label">
                                        <span>QA</span>
                                        <span>78%</span>
                                    </div>
                                    <div class="employee-detail-progress-bar">
                                        <div class="employee-detail-progress-value average" style="width: 78%;"></div>
                                    </div>
                                </div>
                                <div class="employee-detail-progress">
                                    <div class="employee-detail-progress-label">
                                        <span>Management</span>
                                        <span>82%</span>
                                    </div>
                                    <div class="employee-detail-progress-bar">
                                        <div class="employee-detail-progress-value good" style="width: 82%;"></div>
                                    </div>
                                </div>
                            </div>

                            <!-- Company Growth -->
                            <div class="employee-detail-card">
                                <div class="employee-detail-card-title">Company Growth</div>
                                <div class="employee-detail-progress">
                                    <div class="employee-detail-progress-label">
                                        <span>Revenue Growth</span>
                                        <span>+18%</span>
                                    </div>
                                    <div class="employee-detail-progress-bar">
                                        <div class="employee-detail-progress-value good" style="width: 85%;"></div>
                                    </div>
                                </div>
                                <div class="employee-detail-progress">
                                    <div class="employee-detail-progress-label">
                                        <span>Team Growth</span>
                                        <span>+25%</span>
                                    </div>
                                    <div class="employee-detail-progress-bar">
                                        <div class="employee-detail-progress-value good" style="width: 90%;"></div>
                                    </div>
                                </div>
                                <div class="employee-detail-progress">
                                    <div class="employee-detail-progress-label">
                                        <span>Market Share</span>
                                        <span>+12%</span>
                                    </div>
                                    <div class="employee-detail-progress-bar">
                                        <div class="employee-detail-progress-value average" style="width: 65%;"></div>
                                    </div>
                                </div>
                                <div class="employee-detail-progress">
                                    <div class="employee-detail-progress-label">
                                        <span>Innovation Index</span>
                                        <span>72%</span>
                                    </div>
                                    <div class="employee-detail-progress-bar">
                                        <div class="employee-detail-progress-value average" style="width: 72%;"></div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Department Overview -->
                    <div class="employee-detail-section">
                        <div class="employee-detail-section-title">Department Overview</div>

                        <div class="card" style="margin-bottom: 1rem;">
                            <div class="card-header">
                                <div class="card-title">
                                    <span class="card-title-icon">📊</span>
                                    <span>Department Performance</span>
                                </div>
                            </div>
                            <div style="padding: 1rem;">
                                <div class="table-container">
                                    <table>
                                        <thead>
                                        <tr>
                                            <th>Department</th>
                                            <th>Headcount</th>
                                            <th>Productivity</th>
                                            <th>Morale</th>
                                            <th>Budget</th>
                                            <th>Utilization</th>
                                        </tr>
                                        </thead>
                                        <tbody>
                                        <tr>
                                            <td>
                                                <div class="employee-info">
                                                    <span class="card-title-icon">💻</span>
                                                    <span>Development</span>
                                                </div>
                                            </td>
                                            <td>12</td>
                                            <td>
                                                <div class="progress-container">
                                                    <div class="progress-bar">
                                                        <div class="progress-value" style="width: 87%;"></div>
                                                    </div>
                                                    <span class="progress-text">87%</span>
                                                </div>
                                            </td>
                                            <td>
                                                <div class="progress-container">
                                                    <div class="progress-bar">
                                                        <div class="progress-value" style="width: 72%;"></div>
                                                    </div>
                                                    <span class="progress-text">72%</span>
                                                </div>
                                            </td>
                                            <td>$4.8M</td>
                                            <td>
                                                <div class="progress-container">
                                                    <div class="progress-bar">
                                                        <div class="progress-value" style="width: 92%;"></div>
                                                    </div>
                                                    <span class="progress-text">92%</span>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <td>
                                                <div class="employee-info">
                                                    <span class="card-title-icon ba">👥</span>
                                                    <span>Business Analysis</span>
                                                </div>
                                            </td>
                                            <td>5</td>
                                            <td>
                                                <div class="progress-container">
                                                    <div class="progress-bar">
                                                        <div class="progress-value" style="width: 92%;"></div>
                                                    </div>
                                                    <span class="progress-text">92%</span>
                                                </div>
                                            </td>
                                            <td>
                                                <div class="progress-container">
                                                    <div class="progress-bar">
                                                        <div class="progress-value" style="width: 85%;"></div>
                                                    </div>
                                                    <span class="progress-text">85%</span>
                                                </div>
                                            </td>
                                            <td>$1.2M</td>
                                            <td>
                                                <div class="progress-container">
                                                    <div class="progress-bar">
                                                        <div class="progress-value" style="width: 88%;"></div>
                                                    </div>
                                                    <span class="progress-text">88%</span>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <td>
                                                <div class="employee-info">
                                                    <span class="card-title-icon qa">📊</span>
                                                    <span>Quality Assurance</span>
                                                </div>
                                            </td>
                                            <td>4</td>
                                            <td>
                                                <div class="progress-container">
                                                    <div class="progress-bar">
                                                        <div class="progress-value" style="width: 78%;"></div>
                                                    </div>
                                                    <span class="progress-text">78%</span>
                                                </div>
                                            </td>
                                            <td>
                                                <div class="progress-container">
                                                    <div class="progress-bar">
                                                        <div class="progress-value" style="width: 68%;"></div>
                                                    </div>
                                                    <span class="progress-text">68%</span>
                                                </div>
                                            </td>
                                            <td>$0.9M</td>
                                            <td>
                                                <div class="progress-container">
                                                    <div class="progress-bar">
                                                        <div class="progress-value" style="width: 75%;"></div>
                                                    </div>
                                                    <span class="progress-text">75%</span>
                                                </div>
                                            </td>
                                        </tr>
                                        <tr>
                                            <td>
                                                <div class="employee-info">
                                                    <span>🧠</span>
                                                    <span>Research & Innovation</span>
                                                </div>
                                            </td>
                                            <td>3</td>
                                            <td>
                                                <div class="progress-container">
                                                    <div class="progress-bar">
                                                        <div class="progress-value" style="width: 82%;"></div>
                                                    </div>
                                                    <span class="progress-text">82%</span>
                                                </div>
                                            </td>
                                            <td>
                                                <div class="progress-container">
                                                    <div class="progress-bar">
                                                        <div class="progress-value" style="width: 95%;"></div>
                                                    </div>
                                                    <span class="progress-text">95%</span>
                                                </div>
                                            </td>
                                            <td>$1.3M</td>
                                            <td>
                                                <div class="progress-container">
                                                    <div class="progress-bar">
                                                        <div class="progress-value" style="width: 65%;"></div>
                                                    </div>
                                                    <span class="progress-text">65%</span>
                                                </div>
                                            </td>
                                        </tr>
                                        </tbody>
                                    </table>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Company Timeline -->
                    <div class="employee-detail-section">
                        <div class="employee-detail-section-title">Company Timeline</div>

                        <div class="employee-detail-history">
                            <div class="history-item">
                                <div class="history-date">Day 1</div>
                                <div class="history-description">
                                    <strong>Company Founded</strong> - DevCorp Inc. was established with an initial team of 5 developers.
                                </div>
                            </div>
                            <div class="history-item">
                                <div class="history-date">Day 10</div>
                                <div class="history-description">
                                    <strong>First Client</strong> - Secured first major client contract worth $250K.
                                </div>
                            </div>
                            <div class="history-item">
                                <div class="history-date">Day 15</div>
                                <div class="history-description">
                                    <strong>Team Expansion</strong> - Hired 3 additional developers and 2 QA specialists.
                                </div>
                            </div>
                            <div class="history-item">
                                <div class="history-date">Day 22</div>
                                <div class="history-description">
                                    <strong>Office Upgrade</strong> - Moved to a larger office space to accommodate growing team.
                                </div>
                            </div>
                            <div class="history-item">
                                <div class="history-date">Day 30</div>
                                <div class="history-description">
                                    <strong>First Project Completion</strong> - Successfully delivered first major project ahead of schedule.
                                </div>
                            </div>
                            <div class="history-item">
                                <div class="history-date">Day 35</div>
                                <div class="history-description">
                                    <strong>Research Division</strong> - Established dedicated R&D team for innovation.
                                </div>
                            </div>
                            <div class="history-item">
                                <div class="history-date">Day 42</div>
                                <div class="history-description">
                                    <strong>Current Day</strong> - Company continues to grow with 24 employees and 5 active projects.
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </main>
    </div>
</div>
<script>
    // Get all sidebar menu buttons
    const sidebarButtons = document.querySelectorAll('.sidebar-menu-button');

    // Add click event listeners to each button
    sidebarButtons.forEach(button => {
        button.addEventListener('click', function() {
            // Remove active class from all buttons
            sidebarButtons.forEach(btn => btn.classList.remove('active'));

            // Add active class to clicked button
            this.classList.add('active');

            // Get the text content of the button
            const buttonText = this.querySelector('span:last-child').textContent.trim();

            // Check which button was clicked and select the appropriate tab
            if (buttonText === 'Dashboard') {
                document.getElementById('dashboard-tab').checked = true;
            } else if (buttonText === 'Company Overview') {
                document.getElementById('company-overview-tab').checked = true;
            } else if (buttonText === 'Project Overview') {
                document.getElementById('project-overview-tab').checked = true;
            }
        });
    });
</script>
</body>
</html>
