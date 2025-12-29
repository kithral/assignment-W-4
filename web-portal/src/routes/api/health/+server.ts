// Health check endpoint for Kubernetes
import { json } from '@sveltejs/kit';

export async function GET() {
	return json({
		status: 'healthy',
		timestamp: new Date().toISOString(),
		service: 'web-portal'
	});
}
