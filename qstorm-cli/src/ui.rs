use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::Marker,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, AppState};

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Charts
            Constraint::Length(3), // Footer/status
        ])
        .split(frame.area());

    render_header(frame, chunks[0], app);
    render_charts(frame, chunks[1], app);
    render_footer(frame, chunks[2], app);
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let state_text = match app.state {
        AppState::Idle => "IDLE".to_string(),
        AppState::Connecting => "CONNECTING...".to_string(),
        AppState::Warming => "WARMING UP...".to_string(),
        AppState::Running => "RUNNING".to_string(),
        AppState::Paused => "PAUSED".to_string(),
        AppState::Error => "ERROR".to_string(),
    };

    let state_color = match app.state {
        AppState::Running => Color::Green,
        AppState::Paused => Color::Yellow,
        AppState::Error => Color::Red,
        _ => Color::White,
    };

    let header = Paragraph::new(Line::from(vec![
        Span::raw("qstorm "),
        Span::styled(
            format!("[{}]", app.provider_name()),
            Style::default().fg(Color::Cyan),
        ),
        Span::raw(format!(" ({} queries) - ", app.query_count())),
        Span::styled(state_text, Style::default().fg(state_color).bold()),
    ]))
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, area);
}

fn render_charts(frame: &mut Frame, area: Rect, app: &App) {
    // 2x2 grid of charts
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let top_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[0]);

    let bottom_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[1]);

    render_qps_chart(frame, top_row[0], app);
    render_latency_chart(frame, top_row[1], app);
    render_p99_chart(frame, bottom_row[0], app);
    render_recall_chart(frame, bottom_row[1], app);
}

fn render_qps_chart(frame: &mut Frame, area: Rect, app: &App) {
    let data = app.history.qps_series();
    let max_y = data.iter().map(|(_, y)| *y).fold(0.0_f64, f64::max).max(1.0);

    let dataset = Dataset::default()
        .name("QPS")
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan))
        .data(&data);

    let chart = Chart::new(vec![dataset])
        .block(
            Block::default()
                .title(" Queries/Second ")
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .bounds([0.0, data.len().max(1) as f64])
                .labels::<Vec<Span>>(vec![]),
        )
        .y_axis(
            Axis::default()
                .bounds([0.0, max_y * 1.1])
                .labels(vec![
                    Span::raw("0"),
                    Span::raw(format!("{:.0}", max_y / 2.0)),
                    Span::raw(format!("{:.0}", max_y)),
                ]),
        );

    frame.render_widget(chart, area);
}

fn render_latency_chart(frame: &mut Frame, area: Rect, app: &App) {
    let p50_data = app.history.p50_series();
    let max_y = p50_data
        .iter()
        .map(|(_, y)| *y)
        .fold(0.0_f64, f64::max)
        .max(1.0);

    let dataset = Dataset::default()
        .name("p50")
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Green))
        .data(&p50_data);

    let chart = Chart::new(vec![dataset])
        .block(
            Block::default()
                .title(" Latency p50 (ms) ")
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .bounds([0.0, p50_data.len().max(1) as f64])
                .labels::<Vec<Span>>(vec![]),
        )
        .y_axis(
            Axis::default()
                .bounds([0.0, max_y * 1.1])
                .labels(vec![
                    Span::raw("0"),
                    Span::raw(format!("{:.1}", max_y / 2.0)),
                    Span::raw(format!("{:.1}", max_y)),
                ]),
        );

    frame.render_widget(chart, area);
}

fn render_p99_chart(frame: &mut Frame, area: Rect, app: &App) {
    let p99_data = app.history.p99_series();
    let max_y = p99_data
        .iter()
        .map(|(_, y)| *y)
        .fold(0.0_f64, f64::max)
        .max(1.0);

    let dataset = Dataset::default()
        .name("p99")
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Red))
        .data(&p99_data);

    let chart = Chart::new(vec![dataset])
        .block(
            Block::default()
                .title(" Latency p99 (ms) ")
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .bounds([0.0, p99_data.len().max(1) as f64])
                .labels::<Vec<Span>>(vec![]),
        )
        .y_axis(
            Axis::default()
                .bounds([0.0, max_y * 1.1])
                .labels(vec![
                    Span::raw("0"),
                    Span::raw(format!("{:.1}", max_y / 2.0)),
                    Span::raw(format!("{:.1}", max_y)),
                ]),
        );

    frame.render_widget(chart, area);
}

fn render_recall_chart(frame: &mut Frame, area: Rect, app: &App) {
    let recall_data = app.history.recall_series();

    if recall_data.is_empty() {
        // Show placeholder when no recall data
        let placeholder = Paragraph::new("Recall@k\n(no ground truth)")
            .block(
                Block::default()
                    .title(" Recall@k (%) ")
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::DarkGray))
            .wrap(Wrap { trim: true });
        frame.render_widget(placeholder, area);
        return;
    }

    let dataset = Dataset::default()
        .name("recall")
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Magenta))
        .data(&recall_data);

    let chart = Chart::new(vec![dataset])
        .block(
            Block::default()
                .title(" Recall@k (%) ")
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .bounds([0.0, recall_data.len().max(1) as f64])
                .labels::<Vec<Span>>(vec![]),
        )
        .y_axis(
            Axis::default()
                .bounds([0.0, 100.0])
                .labels(vec![
                    Span::raw("0"),
                    Span::raw("50"),
                    Span::raw("100"),
                ]),
        );

    frame.render_widget(chart, area);
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let latest = app.history.latest();

    let stats = if let Some(m) = latest {
        format!(
            "QPS: {:.1} | p50: {:.2}ms | p99: {:.2}ms | Success: {} | Failed: {}",
            m.qps,
            m.latency.p50_us as f64 / 1000.0,
            m.latency.p99_us as f64 / 1000.0,
            m.success_count,
            m.failure_count,
        )
    } else {
        "Waiting for data...".to_string()
    };

    let footer = Paragraph::new(Line::from(vec![
        Span::raw(stats),
        Span::raw(" | "),
        Span::styled("[Space]", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" Pause "),
        Span::styled("[q]", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" Quit"),
    ]))
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}