# Web Portal Documentation

## Overview

The Assignment W-4 web portal is a SvelteKit-based frontend application that provides user registration, authentication, and account management capabilities. It communicates with the Rust microservices backend to handle user accounts, characters, and world access.

## Table of Contents

- [Architecture](#architecture)
- [Technology Stack](#technology-stack)
- [Getting Started](#getting-started)
- [Development](#development)
- [Configuration](#configuration)
- [Features](#features)
- [API Integration](#api-integration)
- [Deployment](#deployment)
- [Testing](#testing)
- [Troubleshooting](#troubleshooting)

## Architecture

The web portal follows a modern JAMstack architecture:

- **Frontend Framework**: SvelteKit with server-side rendering (SSR)
- **Styling**: Tailwind CSS for utility-first styling
- **Validation**: Zod for schema validation
- **State Management**: Svelte stores for reactive state
- **API Communication**: RESTful API client with JWT authentication
- **Build Target**: Node.js adapter for containerized deployment

### Directory Structure

```
web-portal/
├── src/
│   ├── lib/
│   │   ├── api.ts              # API client for backend communication
│   │   ├── stores/
│   │   │   └── auth.ts         # Authentication state management
│   │   └── validation.ts       # Zod validation schemas
│   ├── routes/
│   │   ├── +layout.svelte      # Main layout with navigation
│   │   ├── +page.svelte        # Homepage
│   │   ├── login/
│   │   │   └── +page.svelte    # Login page
│   │   ├── register/
│   │   │   └── +page.svelte    # Registration page
│   │   ├── dashboard/
│   │   │   └── +page.svelte    # User dashboard
│   │   ├── worlds/
│   │   │   └── +page.svelte    # World browser
│   │   └── api/
│   │       └── health/
│   │           └── +server.ts  # Health check endpoint
│   ├── app.html                # HTML template
│   └── app.css                 # Global styles with Tailwind
├── static/                     # Static assets
├── Dockerfile                  # Multi-stage production build
├── .dockerignore              # Docker ignore patterns
├── .env.example               # Environment variable template
├── package.json               # Dependencies and scripts
├── svelte.config.js           # SvelteKit configuration
├── tailwind.config.js         # Tailwind CSS configuration
├── tsconfig.json              # TypeScript configuration
└── vite.config.ts             # Vite dev server configuration
```

## Technology Stack

### Core Dependencies

- **SvelteKit**: ^2.0.0 - Full-stack framework with SSR/SSG/CSR
- **@sveltejs/adapter-node**: ^5.0.1 - Node.js production adapter
- **Svelte**: ^4.0.0 - Reactive UI framework
- **Vite**: ^5.0.0 - Fast build tool and dev server

### Styling

- **Tailwind CSS**: ^3.4.0 - Utility-first CSS framework
- **PostCSS**: ^8.4.0 - CSS transformations
- **Autoprefixer**: ^10.4.0 - CSS vendor prefixing

### Validation & TypeScript

- **Zod**: ^3.22.4 - TypeScript-first schema validation
- **TypeScript**: ^5.0.0 - Type-safe JavaScript

## Getting Started

### Prerequisites

- Node.js 20.x or later
- npm 10.x or later
- Running auth-service backend (for authentication)

### Installation

1. Navigate to the web-portal directory:

```bash
cd web-portal
```

1. Install dependencies:

```bash
npm install
```

1. Copy the environment template:

```bash
cp .env.example .env
```

1. Configure environment variables in `.env`:

```env
# API endpoint for backend services
VITE_API_URL=http://localhost:8084

# Optional: Public URL for the portal
PUBLIC_URL=http://localhost:5173
```

1. Start the development server:

```bash
npm run dev
```

The portal will be available at `http://localhost:5173`.

## Development

### Available Scripts

- `npm run dev` - Start development server with hot reload
- `npm run build` - Build production bundle
- `npm run preview` - Preview production build locally
- `npm run check` - Run SvelteKit checks
- `npm run check:watch` - Run checks in watch mode
- `npm run lint` - Lint code (if configured)
- `npm run format` - Format code (if configured)

### Development Workflow

1. **Make changes** to Svelte components or API client
2. **Hot reload** updates automatically in the browser
3. **Type checking** runs in the background
4. **Test locally** with the development server
5. **Build production** bundle to verify

### Adding New Pages

Create a new route by adding a `+page.svelte` file:

```svelte
<!-- src/routes/my-page/+page.svelte -->
<script lang="ts">
  // Component logic
</script>

<svelte:head>
  <title>My Page - Assignment W-4</title>
</svelte:head>

<div class="container mx-auto px-4 py-8">
  <h1 class="text-3xl font-bold">My Page</h1>
  <!-- Page content -->
</div>
```

### Adding API Endpoints

Create server-side endpoints with `+server.ts`:

```typescript
// src/routes/api/my-endpoint/+server.ts
import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ locals }) => {
  return json({
    message: 'Success',
    data: {}
  });
};
```

## Configuration

### Environment Variables

#### Development (VITE_* prefix)

```env
# Backend API URL (browser-side)
VITE_API_URL=http://localhost:8084

# Optional public URL
PUBLIC_URL=http://localhost:5173
```

#### Production (no prefix)

```env
# Backend API URL (server-side)
API_URL=http://auth-service:8080

# Node environment
NODE_ENV=production

# Server configuration
PORT=3000
HOST=0.0.0.0
```

### SvelteKit Configuration

Edit `svelte.config.js` to customize:

```javascript
import adapter from '@sveltejs/adapter-node';

const config = {
  kit: {
    adapter: adapter({
      out: 'build',           // Output directory
      precompress: true,      // Enable gzip/brotli compression
      envPrefix: ''           // Environment variable prefix
    }),
    alias: {
      $components: 'src/components',
      $utils: 'src/lib/utils'
    }
  }
};
```

### Tailwind Configuration

Customize Tailwind in `tailwind.config.js`:

```javascript
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        // Add custom colors
        primary: '#4F46E5',
        secondary: '#10B981'
      }
    }
  },
  plugins: [
    // Add Tailwind plugins
  ]
};
```

## Features

### Authentication

#### Login Flow

1. User enters credentials on `/login`
2. Form validation with Zod schema
3. API call to `POST /api/auth/login`
4. JWT token stored in localStorage
5. User state updated in auth store
6. Redirect to dashboard

#### Registration Flow

1. User fills registration form on `/register`
2. Validation requirements:
   - Username: 3-20 characters, alphanumeric + underscore
   - Email: Valid email format
   - Password: Minimum 8 characters, uppercase, lowercase, number
   - Confirm password: Must match
3. API call to `POST /api/auth/register`
4. Automatic login after successful registration
5. Redirect to dashboard

#### Protected Routes

Protected routes check authentication status:

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { authStore } from '$lib/stores/auth';

  onMount(() => {
    const unsubscribe = authStore.subscribe(state => {
      if (!state.token) {
        goto('/login');
      }
    });
    return unsubscribe;
  });
</script>
```

### Dashboard

User dashboard displays:

- **Account Information**: Username, email, member since
- **Character List**: Characters with level and playtime
- **Recent Worlds**: Worlds with status and player count
- **Quick Stats**: Level, characters, worlds visited, playtime

### Form Validation

All forms use Zod schemas for validation:

```typescript
// Login validation
const loginSchema = z.object({
  username: z.string()
    .min(3, 'Username must be at least 3 characters')
    .max(50, 'Username must be less than 50 characters'),
  password: z.string()
    .min(8, 'Password must be at least 8 characters')
});

// Registration validation
const registerSchema = z.object({
  username: z.string()
    .min(3).max(20)
    .regex(/^[a-zA-Z0-9_]+$/, 'Username can only contain letters, numbers, and underscores'),
  email: z.string()
    .email('Please enter a valid email address'),
  password: z.string()
    .min(8, 'Password must be at least 8 characters')
    .regex(/[A-Z]/, 'Password must contain at least one uppercase letter')
    .regex(/[a-z]/, 'Password must contain at least one lowercase letter')
    .regex(/[0-9]/, 'Password must contain at least one number'),
  confirmPassword: z.string()
}).refine((data) => data.password === data.confirmPassword, {
  message: "Passwords don't match",
  path: ["confirmPassword"]
});
```

## API Integration

### API Client

The API client (`src/lib/api.ts`) handles all backend communication:

```typescript
import { browser } from '$app/environment';

// Environment-based URL selection
const API_BASE_URL = browser
  ? (import.meta.env.VITE_API_URL || 'http://localhost:8084')
  : (process.env.API_URL || 'http://auth-service:8080');

class ApiClient {
  private baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
  }

  // Generic request wrapper
  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<ApiResponse<T>> {
    const url = `${this.baseUrl}${endpoint}`;
    const token = browser ? localStorage.getItem('auth_token') : null;

    const headers: HeadersInit = {
      'Content-Type': 'application/json',
      ...options.headers,
    };

    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    }

    try {
      const response = await fetch(url, { ...options, headers });
      const data = await response.json();

      if (!response.ok) {
        return {
          success: false,
          error: data.error || 'Request failed'
        };
      }

      return {
        success: true,
        data
      };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Network error'
      };
    }
  }

  // Authentication methods
  async login(credentials: LoginRequest): Promise<ApiResponse<AuthResponse>> {
    return this.request<AuthResponse>('/api/auth/login', {
      method: 'POST',
      body: JSON.stringify(credentials)
    });
  }

  async register(data: RegisterRequest): Promise<ApiResponse<AuthResponse>> {
    return this.request<AuthResponse>('/api/auth/register', {
      method: 'POST',
      body: JSON.stringify(data)
    });
  }

  async logout(): Promise<ApiResponse<void>> {
    return this.request<void>('/api/auth/logout', {
      method: 'POST'
    });
  }

  async getCurrentUser(): Promise<ApiResponse<User>> {
    return this.request<User>('/api/users/me');
  }

  async refreshToken(): Promise<ApiResponse<AuthResponse>> {
    return this.request<AuthResponse>('/api/auth/refresh', {
      method: 'POST'
    });
  }
}

export const apiClient = new ApiClient(API_BASE_URL);
```

### TypeScript Interfaces

```typescript
export interface User {
  id: string;
  username: string;
  email: string;
  created_at: string;
  updated_at: string;
}

export interface LoginRequest {
  username: string;
  password: string;
}

export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}
```

### State Management

Authentication state uses Svelte stores:

```typescript
import { writable } from 'svelte/store';
import { browser } from '$app/environment';

interface AuthState {
  user: User | null;
  token: string | null;
  loading: boolean;
}

function createAuthStore() {
  const { subscribe, set, update } = writable<AuthState>({
    user: null,
    token: browser ? localStorage.getItem('auth_token') : null,
    loading: false
  });

  return {
    subscribe,
    setUser: (user: User, token: string) => {
      if (browser) {
        localStorage.setItem('auth_token', token);
      }
      set({ user, token, loading: false });
    },
    clearUser: () => {
      if (browser) {
        localStorage.removeItem('auth_token');
      }
      set({ user: null, token: null, loading: false });
    },
    setLoading: (loading: boolean) => {
      update(state => ({ ...state, loading }));
    }
  };
}

export const authStore = createAuthStore();
```

Usage in components:

```svelte
<script lang="ts">
  import { authStore } from '$lib/stores/auth';

  // Subscribe to auth state
  $: user = $authStore.user;
  $: isAuthenticated = !!$authStore.token;
</script>

{#if isAuthenticated}
  <p>Welcome, {user?.username}!</p>
{:else}
  <a href="/login">Login</a>
{/if}
```

## Deployment

### Docker Build

Build the Docker image:

```bash
docker build -t web-portal:latest -f web-portal/Dockerfile ./web-portal
```

Run locally:

```bash
docker run -p 3000:3000 \
  -e API_URL=http://auth-service:8080 \
  -e NODE_ENV=production \
  web-portal:latest
```

### Kubernetes Deployment

The web portal is deployed via Kustomize and ArgoCD:

```bash
# Apply directly with kubectl
kubectl apply -k deployments/web-portal

# Or let ArgoCD sync automatically
# (if ApplicationSet is configured)
```

#### Deployment Configuration

```yaml
# deployments/web-portal/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: deployment
spec:
  replicas: 2
  template:
    spec:
      containers:
      - name: web-portal
        image: web-portal:latest
        ports:
        - name: http
          containerPort: 3000
        env:
        - name: NODE_ENV
          value: production
        - name: API_URL
          value: http://auth-service:8080
        resources:
          requests:
            memory: "128Mi"
            cpu: "100m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /api/health
            port: http
          initialDelaySeconds: 10
          periodSeconds: 30
        readinessProbe:
          httpGet:
            path: /api/health
            port: http
          initialDelaySeconds: 5
          periodSeconds: 10
```

#### Service Configuration

```yaml
# deployments/web-portal/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: service
spec:
  type: ClusterIP
  ports:
  - name: http
    port: 3000
    targetPort: http
  selector:
    app: web-portal
```

#### Ingress Configuration

```yaml
# deployments/web-portal/ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: ingress
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
spec:
  ingressClassName: nginx
  rules:
  - host: assignment-w4.local
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: web-portal-service
            port:
              number: 3000
```

### GitHub Actions CI/CD

The web portal is built and deployed automatically via GitHub Actions:

```yaml
# .github/workflows/build-deploy.yaml
jobs:
  detect-changes:
    runs-on: ubuntu-latest
    outputs:
      web-portal: ${{ steps.filter.outputs.web-portal }}
    steps:
      - uses: dorny/paths-filter@v2
        id: filter
        with:
          filters: |
            web-portal:
              - 'web-portal/**'
              - 'deployments/web-portal/**'

  build-web-portal:
    needs: detect-changes
    if: needs.detect-changes.outputs.web-portal == 'true'
    runs-on: ubuntu-latest
    steps:
      - name: Build and push Docker image
        # ... build steps
      - name: Update Kustomize image
        # ... kustomize edit set image
```

### ArgoCD Integration

The web portal is included in the microservices ApplicationSet:

```yaml
# argocd/applicationsets/microservices.yaml
spec:
  generators:
  - list:
      elements:
      # ... other services
      - service: web-portal
        replicas: 2
```

ArgoCD will automatically:
- Detect changes to `deployments/web-portal/**`
- Sync the deployment
- Update running pods with new images
- Monitor health status

## Testing

### Local Testing

Test the production build locally:

```bash
# Build production bundle
npm run build

# Preview the build
npm run preview
```

Access at `http://localhost:4173`

### Health Check

Verify the health endpoint:

```bash
curl http://localhost:3000/api/health
```

Expected response:

```json
{
  "status": "healthy",
  "timestamp": "2025-12-29T12:00:00.000Z",
  "service": "web-portal"
}
```

### Integration Testing

Test API integration with backend:

```bash
# Start auth-service locally
cd services/auth-service
cargo run

# Start web portal
cd web-portal
npm run dev

# Test login flow
curl -X POST http://localhost:8084/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"test","password":"password123"}'
```

### Kubernetes Health Checks

Verify health checks in Kubernetes:

```bash
# Check pod status
kubectl get pods -l app=web-portal

# Check logs
kubectl logs -l app=web-portal --tail=50

# Test readiness probe
kubectl exec -it <pod-name> -- wget -qO- http://localhost:3000/api/health
```

## Troubleshooting

### Common Issues

#### 1. API Connection Failures

**Symptom**: Login/registration fails with network errors

**Solutions**:

- Verify `VITE_API_URL` is set correctly in `.env`
- Check auth-service is running: `kubectl get pods -l app=auth-service`
- Check service DNS: `kubectl exec -it <web-portal-pod> -- nslookup auth-service`
- Review CORS configuration on auth-service

#### 2. Authentication State Not Persisting

**Symptom**: User is logged out on page refresh

**Solutions**:

- Check browser console for localStorage errors
- Verify token is being stored: `localStorage.getItem('auth_token')`
- Check token expiration time
- Review auth store initialization

#### 3. Build Failures

**Symptom**: `npm run build` fails

**Solutions**:

```bash
# Clear cache and reinstall
rm -rf node_modules .svelte-kit
npm install

# Check TypeScript errors
npm run check

# Verify Vite config
npm run build -- --debug
```

#### 4. Docker Build Failures

**Symptom**: Docker build fails during `npm ci`

**Solutions**:

```bash
# Clear Docker cache
docker builder prune

# Build with no cache
docker build --no-cache -t web-portal:latest ./web-portal

# Check package-lock.json exists
ls -la web-portal/package-lock.json
```

#### 5. Pod CrashLoopBackOff

**Symptom**: Web portal pods keep restarting

**Solutions**:

```bash
# Check logs
kubectl logs -l app=web-portal --previous

# Verify environment variables
kubectl get configmap web-portal-config -o yaml

# Check resource limits
kubectl describe pod -l app=web-portal

# Test health endpoint manually
kubectl port-forward svc/web-portal-service 3000:3000
curl http://localhost:3000/api/health
```

#### 6. Ingress Not Accessible

**Symptom**: Cannot access portal via domain

**Solutions**:

```bash
# Check ingress status
kubectl get ingress web-portal-ingress

# Verify NGINX ingress controller
kubectl get pods -n ingress-nginx

# Check ingress logs
kubectl logs -n ingress-nginx -l app.kubernetes.io/component=controller

# Update /etc/hosts if using local domain
echo "192.168.1.100 assignment-w4.local" | sudo tee -a /etc/hosts
```

### Debug Mode

Enable debug logging:

```typescript
// src/lib/api.ts
const DEBUG = import.meta.env.DEV;

private async request<T>(endpoint: string, options: RequestInit = {}) {
  if (DEBUG) {
    console.log('[API]', endpoint, options);
  }
  // ... rest of request
}
```

### Performance Profiling

Profile the application:

```bash
# Build with source maps
npm run build

# Analyze bundle size
npx vite-bundle-visualizer
```

## Best Practices

### Security

1. **Never commit secrets** - Use `.env` files, never commit them
2. **Validate all inputs** - Use Zod schemas for form validation
3. **Sanitize user input** - Prevent XSS attacks
4. **Use HTTPS** - Always use TLS in production
5. **Set security headers** - Configure CSP, HSTS, etc.
6. **Token expiration** - Implement refresh token flow

### Performance

1. **Code splitting** - Lazy load routes and components
2. **Image optimization** - Use WebP format, lazy loading
3. **Caching** - Configure proper cache headers
4. **Bundle size** - Keep dependencies minimal
5. **SSR when beneficial** - Use for SEO-critical pages

### Accessibility

1. **Semantic HTML** - Use proper HTML5 elements
2. **ARIA labels** - Add labels for screen readers
3. **Keyboard navigation** - Support tab navigation
4. **Color contrast** - Ensure sufficient contrast ratios
5. **Focus indicators** - Visible focus states

### Development

1. **Type safety** - Use TypeScript throughout
2. **Component reusability** - Create shared components
3. **Consistent styling** - Follow Tailwind patterns
4. **Error boundaries** - Handle errors gracefully
5. **Documentation** - Comment complex logic

## Next Steps

### Planned Features

1. **Email Verification** - Verify email addresses on registration
2. **Password Reset** - Forgot password flow
3. **Profile Management** - Update username, email, password
4. **Character Management** - Create, edit, delete characters
5. **World Browser** - Browse and join available worlds
6. **Real-time Updates** - WebSocket connection for live updates
7. **Admin Dashboard** - Administrative tools
8. **Telemetry** - Usage analytics and monitoring

### Integration Roadmap

1. Connect to real auth-service API endpoints
2. Implement persistence-service integration
3. Add world-state service integration
4. Implement chat service integration
5. Add real-time features via WebSocket
6. Implement asset management
7. Add graphics gateway integration

## Resources

- [SvelteKit Documentation](https://kit.svelte.dev/docs)
- [Svelte Tutorial](https://svelte.dev/tutorial)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs)
- [Zod Documentation](https://zod.dev)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [Vite Documentation](https://vitejs.dev/guide/)
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [ArgoCD Documentation](https://argo-cd.readthedocs.io/)
